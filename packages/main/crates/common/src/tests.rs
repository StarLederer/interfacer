use bollard::{container::{Config, RemoveContainerOptions}, exec::CreateExecOptions, image::CreateImageOptions, Docker};
use futures_util::TryStreamExt;

#[tokio::test]
async fn docker_connects() {
    let docker = match Docker::connect_with_local_defaults() {
        Ok(docker) => docker,
        Err(e) => {
            panic!("{}", e);
        }
    };

    println!("{:?}", docker);
}

#[tokio::test]
async fn returns_version() {
    let docker = match Docker::connect_with_local_defaults() {
        Ok(docker) => docker,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let version = docker.version().await.unwrap();
    println!("{:?}", version);
}

#[tokio::test]
async fn creates_images() {
    let docker = match Docker::connect_with_local_defaults() {
        Ok(docker) => docker,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let image = match docker
        .create_image(
            Some(CreateImageOptions {
                from_image: "node:16",
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
    {
        Ok(image) => image,
        Err(e) => {
            panic!("{}", e);
        }
    };

    println!("{:?}", image);
}

#[tokio::test]
async fn runs_containers() {
    let docker = match Docker::connect_with_local_defaults() {
        Ok(docker) => docker,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let container_config = Config {
        image: Some("node:16"),
        tty: Some(true),
        ..Default::default()
    };

    let container = match docker
        .create_container::<&str, &str>(None, container_config)
        .await
    {
        Ok(container) => container,
        Err(e) => {
            panic!("{}", e);
        }
    };

    docker
        .start_container::<String>(&container.id, None)
        .await
        .unwrap();

    println!("{:?}", container);

    let exec = match docker
        .create_exec(
            &container.id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["ls", "-l", "/"]),
                ..Default::default()
            },
        )
        .await
    {
        Ok(exec) => exec,
        Err(e) => {
            panic!("{}", e);
        }
    };

    println!("{:?}", exec);

    docker
        .remove_container(
            &container.id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await
        .unwrap();
}
