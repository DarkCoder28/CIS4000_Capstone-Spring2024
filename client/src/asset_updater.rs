use git2::{build::CheckoutBuilder, Repository};
use tracing::info;

pub fn update_assets(assets_path: &str) -> Result<(), git2::Error> {
    // Define the repository URL
    let repo_url = "https://github.com/DarkCoder28/CIS4000_Capstone-Spring2024-ASSETS.git"; // Replace with your GitHub repository URL

    // Open the repository or clone it if it doesn't exist
    let repo = match Repository::open(assets_path) {
        Ok(repo) => repo,
        Err(_) => {
            info!("Repository doesn't exist locally. Cloning...");
            Repository::clone(repo_url, assets_path)?
        }
    };

    // Get the remote named "origin"
    let mut remote = repo.find_remote("origin")?;

    // Fetch the latest changes
    let refspecs = ["refs/heads/*:refs/heads/*"];
    remote.fetch(&refspecs, None, None)?;

    // Get the remote branch to compare against
    let remote_branch_name = "origin/master";
    let remote_branch = repo.find_branch(remote_branch_name, git2::BranchType::Remote)?;
    let remote_commit = remote_branch.get().peel_to_commit()?;

    // Reset the local branch to match the remote
    let obj = repo.find_object(remote_commit.id(), None)?;
    repo.reset(&obj, git2::ResetType::Hard, CheckoutBuilder::new().force().into())?;//Some(&mut branch_ref.name()))?;

    println!("Local branch reset to match remote");

    Ok(())
}
