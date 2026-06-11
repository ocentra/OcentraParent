const token = process.env.GITHUB_TOKEN;
const repository = process.env.GITHUB_REPOSITORY;
const eventName = process.env.GITHUB_EVENT_NAME;
const prNumber = process.env.PR_NUMBER;

if (eventName !== 'pull_request') {
  console.log('PR review thread gate skipped outside pull_request events.');
  process.exit(0);
}

if (!token || !repository || !prNumber) {
  throw new Error('GITHUB_TOKEN, GITHUB_REPOSITORY, and PR_NUMBER are required for PR review thread checks.');
}

const [owner, repo] = repository.split('/');
if (!owner || !repo) {
  throw new Error(`Invalid GITHUB_REPOSITORY value: ${repository}`);
}

const unresolvedThreads = await collectUnresolvedThreads({ owner, repo, number: Number.parseInt(prNumber, 10) });

if (unresolvedThreads.length === 0) {
  console.log('No unresolved PR review threads found.');
  process.exit(0);
}

console.error(`Found ${unresolvedThreads.length} unresolved PR review thread(s). Resolve these before merge:`);
for (const thread of unresolvedThreads) {
  console.error(`- ${thread.path}:${thread.line ?? 'file'} ${thread.title}`);
  console.error(`  ${thread.url}`);
}
process.exit(1);

async function collectUnresolvedThreads(variables) {
  const query = `
    query($owner: String!, $repo: String!, $number: Int!, $cursor: String) {
      repository(owner: $owner, name: $repo) {
        pullRequest(number: $number) {
          reviewThreads(first: 100, after: $cursor) {
            pageInfo {
              hasNextPage
              endCursor
            }
            nodes {
              isResolved
              isOutdated
              path
              line
              comments(first: 1) {
                nodes {
                  body
                  url
                }
              }
            }
          }
        }
      }
    }
  `;
  const unresolved = [];
  let cursor = null;
  do {
    const response = await fetch('https://api.github.com/graphql', {
      method: 'POST',
      headers: {
        authorization: `Bearer ${token}`,
        'content-type': 'application/json',
        'user-agent': 'ocentra-parent-pr-review-thread-gate',
      },
      body: JSON.stringify({ query, variables: { ...variables, cursor } }),
    });
    if (!response.ok) {
      throw new Error(`GitHub GraphQL request failed: ${response.status} ${await response.text()}`);
    }
    const payload = await response.json();
    if (payload.errors?.length) {
      throw new Error(`GitHub GraphQL errors: ${JSON.stringify(payload.errors)}`);
    }
    const threads = payload.data.repository.pullRequest.reviewThreads;
    for (const thread of threads.nodes) {
      if (!thread.isResolved && !thread.isOutdated) {
        unresolved.push(formatThread(thread));
      }
    }
    cursor = threads.pageInfo.hasNextPage ? threads.pageInfo.endCursor : null;
  } while (cursor);
  return unresolved;
}

function formatThread(thread) {
  const comment = thread.comments.nodes[0] ?? {};
  return {
    path: thread.path,
    line: thread.line,
    title: firstMeaningfulLine(comment.body),
    url: comment.url,
  };
}

function firstMeaningfulLine(body = '') {
  return (
    body
      .split('\n')
      .map((line) => line.trim())
      .find((line) => line.length > 0)
      ?.replace(/^#+\s*/u, '') ?? 'Unresolved review thread'
  );
}
