from testcontainers.core.image import Image
from testcontainers.core.container import DockerContainer

def get_container(config: str,
                  image: str = "forge.cptlobster.dev/cptlobster/bene-gesserit:dev",
                  host: str = "") -> DockerContainer:
    container = (
        DockerContainer(image)
            .with_exposed_ports(9999)
            .with_volume_mapping(config, "/etc/bene-gesserit/config.toml", "ro")
    )
    return container

def get_sample_server() -> DockerContainer:
    container = (
        DockerContainer("httpd")
            .with_exposed_ports(80)
            .with_volume_mapping("./example", "/usr/local/apache2/htdocs", "ro")
    )
    return container

def is_valid(response) -> bool:
    pass

def is_labyrinth(response) -> bool:
    pass

def is_ratelimited(response) -> bool:
    pass