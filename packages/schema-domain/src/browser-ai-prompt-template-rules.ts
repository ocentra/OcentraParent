type PromptTemplateVersionRecordShape = {
  readonly status: string;
  readonly validUntil: unknown | null;
  readonly previousPromptTemplateVersion: unknown | null;
  readonly supersededByPromptTemplateVersion: unknown | null;
  readonly invalidatesMemory: boolean;
  readonly inputFieldRefsChanged: boolean;
  readonly changeReasons: readonly string[];
  readonly template: { readonly requestedTask: string };
  readonly compatibleModelRuntimeRefs: readonly string[];
  readonly policyVersionRefs: readonly string[];
};

type PromptTemplateRegistryShape = {
  readonly versions: readonly PromptTemplateVersionRecordShape[];
};

type PromptTemplateSelectionShape = {
  readonly selectionState: string;
  readonly selectedPromptTemplate: unknown | null;
  readonly degradedStates: readonly string[];
  readonly promptChangedInvalidatesMemory: boolean;
};

type PromptTemplateSelectionRequestShape = {
  readonly registry: PromptTemplateRegistryShape;
  readonly requestedTask: string;
  readonly modelRuntimeRef: string;
  readonly policyVersionRef: string;
};

export function browserAiPromptTemplateVersionRecordIsConsistent(value: PromptTemplateVersionRecordShape) {
  return (
    promptTemplateChangeIsMemoryAware(value) &&
    (value.status === 'active'
      ? value.validUntil === null && value.supersededByPromptTemplateVersion === null
      : value.status === 'draft'
        ? value.validUntil === null && value.supersededByPromptTemplateVersion === null
        : value.validUntil !== null && value.supersededByPromptTemplateVersion !== null)
  );
}

export function browserAiPromptTemplateRegistryIsConsistent(value: PromptTemplateRegistryShape) {
  const activeTaskModelKeys = new Set<string>();
  for (const version of value.versions) {
    if (version.status !== 'active') {
      continue;
    }
    for (const modelRuntimeRef of version.compatibleModelRuntimeRefs) {
      const key = `${version.template.requestedTask}:${modelRuntimeRef}`;
      if (activeTaskModelKeys.has(key)) {
        return false;
      }
      activeTaskModelKeys.add(key);
    }
  }
  return true;
}

export function browserAiPromptTemplateSelectionIsConsistent(value: PromptTemplateSelectionShape) {
  return value.selectionState === 'selected'
    ? value.selectedPromptTemplate !== null && value.degradedStates.length === 0
    : value.selectedPromptTemplate === null &&
        value.degradedStates.length > 0 &&
        !value.promptChangedInvalidatesMemory;
}

export function selectBrowserAiPromptTemplateRecord(request: PromptTemplateSelectionRequestShape) {
  const activeByTask = request.registry.versions.filter(
    (version) => version.status === 'active' && version.template.requestedTask === request.requestedTask
  );
  const activeByModel = activeByTask.filter((version) =>
    version.compatibleModelRuntimeRefs.includes(request.modelRuntimeRef)
  );
  const activeByPolicy = activeByModel.filter((version) => version.policyVersionRefs.includes(request.policyVersionRef));
  const selected = activeByPolicy.length === 1 ? activeByPolicy[0] : null;

  return {
    selected,
    degradedStates:
      selected !== null
        ? []
        : activeByTask.length === 0
          ? ['template-missing']
          : activeByModel.length === 0
            ? ['model-unsupported']
            : ['policy-version-unsupported'],
  } as const;
}

function promptTemplateChangeIsMemoryAware(value: PromptTemplateVersionRecordShape) {
  return (
    (value.previousPromptTemplateVersion === null || value.invalidatesMemory) &&
    (!value.inputFieldRefsChanged ||
      (value.invalidatesMemory && value.changeReasons.includes('input-field-change')))
  );
}
