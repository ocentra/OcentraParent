use std::path::{Path, PathBuf};

use ocentra_child_runtime::service::{
    ChildAgentIngressError, ChildAgentReadiness, ChildAgentService, ChildAgentServiceError,
    ChildAgentServicePaths,
};

fn unique_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-child-runtime-device-trust-{label}-{}",
        std::process::id()
    ))
}

fn clean_root(root: &Path) {
    let _ = std::fs::remove_dir_all(root);
}

#[tokio::test]
async fn startup_without_owner_current_binding_remains_manual_required(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("missing-binding");
    clean_root(&root);

    let service =
        ChildAgentService::initialize_with_paths(ChildAgentServicePaths::from_root(root.clone()))
            .await?;

    let health = service.health()?;
    assert_eq!(
        health.readiness,
        ChildAgentReadiness::TrustBindingManualRequired
    );
    assert_eq!(
        service.readiness()?,
        ChildAgentReadiness::TrustBindingManualRequired
    );
    assert_eq!(health.durable_root, root);

    drop(service);
    clean_root(&health.durable_root);
    Ok(())
}

#[tokio::test]
async fn ingress_rejects_observed_commands_until_owner_current_binding_is_available(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("unauthenticated-ingress");
    clean_root(&root);

    let service =
        ChildAgentService::initialize_with_paths(ChildAgentServicePaths::from_root(root.clone()))
            .await?;
    let ingress = service.ingress();
    let service_task = tokio::spawn(service.run_until_shutdown());
    let result = ingress
        .submit_observed_event(ocentra_app_core::app_observed_event(
            ocentra_app_core::AppObservationIntent::InventoryObservationOnly,
        ))
        .await;
    let error = match result {
        Err(error) => error,
        Ok(_) => return Err("in-process ingress accepted a command without current trust".into()),
    };

    let ChildAgentIngressError::Service(error) = error else {
        return Err("missing current trust must be a service readiness rejection".into());
    };
    match *error {
        ChildAgentServiceError::TrustBindingManualRequired => {}
        unexpected => {
            return Err(format!("unexpected missing-trust service error: {unexpected:?}").into())
        }
    }

    service_task.abort();
    let _ = service_task.await;
    clean_root(&root);
    Ok(())
}

#[tokio::test]
async fn missing_owner_binding_stays_manual_required_across_restart(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("restart");
    clean_root(&root);

    let first =
        ChildAgentService::initialize_with_paths(ChildAgentServicePaths::from_root(root.clone()))
            .await?;
    assert_eq!(
        first.readiness()?,
        ChildAgentReadiness::TrustBindingManualRequired
    );
    drop(first);

    let restarted =
        ChildAgentService::initialize_with_paths(ChildAgentServicePaths::from_root(root.clone()))
            .await?;
    assert_eq!(
        restarted.readiness()?,
        ChildAgentReadiness::TrustBindingManualRequired
    );

    drop(restarted);
    clean_root(&root);
    Ok(())
}
