image_name := env_var_or_default("IMAGE_NAME", "pico-env")

# https://stackoverflow.com/questions/23513045/how-to-check-if-a-process-is-running-inside-docker-container
is_in_docker := `test -f /.dockerenv && echo 1 || echo 0`

# run_cmd is the docker run command used to launch your env.
run_cmd := if is_in_docker == "0" {
    'docker run -it --rm --mount type=bind,src="$(pwd)",target="/project" -w /project ' + image_name
} else {""}

#################

# Display the prefix command used in the current context
echo-run-cmd:
    @echo run_cmd={{run_cmd}}

# Check if we are running in the docker container
check-in-docker:
    @echo {{if is_in_docker == "1" {"In docker container!"} else  {"Not in docker container!"} }}

# Build the docker environment used in this project
build-docker force="no":
    #!/usr/bin/env sh
    if [ {{is_in_docker}} -eq 1 ] ; then
        echo "Running in docker container, skipping image generation !"
    else
        if [ -z `docker image ls {{image_name}} --format '1'`] || [ {{force}} = "force" ] ; then
            [ {{force}} = "force" ] && echo "Forcing build"
            docker buildx build --load -t {{image_name}} docker
        fi
    fi



#################

# Build the project
build: build-docker
    {{run_cmd}} cargo build --release

# Open a bash shell in the docker container
shell: build-docker
    {{run_cmd}} /bin/bash

# Call the cargo command in the container
cargo +args: build-docker
    {{run_cmd}} cargo {{args}}
	
# Get the .uf2 file of the main app in the root folder
get-uf2: build
    # TODO # Make this compatible outside of docker container
    {{run_cmd}} /home/builder/.cargo/bin/elf2uf2-rs target/thumbv6m-none-eabi/release/rp-pico-platform rp-pico-platform.uf2
