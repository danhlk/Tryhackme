> # Intro to Docker

# Summary
<!-- TOC -->

- [Summary](#summary)
    - [Task 2 - Basic Docker Syntax](#task-2---basic-docker-syntax)
    - [Task 3 - Running Your First Container](#task-3---running-your-first-container)
    - [Task 4 - Intro to Dockerfiles](#task-4---intro-to-dockerfiles)
    - [Task 5 - Intro to Docker Compose](#task-5---intro-to-docker-compose)
    - [Task 6 - Intro to the Docker Socket](#task-6---intro-to-the-docker-socket)
    - [Task 7 - Practical](#task-7---practical)

<!-- /TOC -->

## Task 2 - Basic Docker Syntax
1. If we wanted to pull a docker image, what would our command look like?<br>
    > Images can be downloaded using the docker pull command and providing the name of the image.

    **Answer:** docker pull

1. If we wanted to list all images on a device running Docker, what would our command look like?<br>
    > This command allows us to list all images stored on the local system.

    **Answer:** docker image ls

1. Let's say we wanted to pull the image "tryhackme" (no quotations); what would our command look like?<br>
    **Answer:** docker pull tryhackme

1. Let's say we wanted to pull the image "tryhackme" with the tag "1337" (no quotations). What would our command look like?<br>
    **Answer:** docker pull tryhackme:1337

## Task 3 - Running Your First Container
1. What would our command look like if we wanted to run a container interactively?<br>
    > "Interactively" by providing the -it switch in the [OPTIONS] command. This will allow us to interact with the container directly.
    **Answer:** docker run -it

1. What would our command look like if we wanted to run a container in "detached" mode?<br>
    > -d: This argument tells the container to start in "detached" mode. This means that the container will run in the background.

    **Answer:** docker run -d

1. Let's say we want to run a container that will run and bind a webserver on port 80. What would our command look like?<br>
    > -p: This argument tells Docker to bind a port on the host operating system to a port that is being exposed in the container. You would use this instruction if you are running an application or service (such as a web server) in the container and wish to access the application/service by navigating to the IP address.

    **Answer:** docker run -p 80:80

1. Now, how would we list all containers (including stopped)?<br>
    > To list all containers (even stopped), you can use docker ps -a

    **Answer:** docker ps -a

## Task 4 - Intro to Dockerfiles
1. What instruction would we use to specify what base image the container should be using?<br>
    > FROM: This instruction sets a build stage for the container as well as setting the base image (operating system). All Dockerfiles must start with this.

    **Answer:** from

1. What instruction would we use to tell the container to run a command?<br>
    > RUN: This instruction will execute commands in the container within a new layer.

    **Answer:** run

1. What docker command would we use to build an image using a Dockerfile?<br>
    **Answer:** build

1. Let's say we want to name this image; what argument would we use?<br>
    > Whether or not you want to name the image yourself (we will use the -t (tag) argument).

    **Answer:** -t

## Task 5 - Intro to Docker Compose
1. I want to use docker-compose  to start up a series of containers. What argument allows me to do this?<br>
    > up: This command will (re)create/build and start the containers specified in the compose file.

    **Answer:** up

1. I want to use docker-compose  to delete the series of containers. What argument allows me to do this?<br>
    > down: This command will stop and delete the containers specified in the compose file.

    **Answer:** down

1. What is the name of the .yml file that docker-compose uses?<br>
    **Answer:** docker-compose.yml


## Task 6 - Intro to the Docker Socket
1. What does the term "IPC" stand for?<br>
    > Interprocess Communication (IPC). This simply means that processes on an operating system can communicate with each other!

    **Answer:** Interprocess Communication

1. What technology can the Docker Server be equalled to?<br>
    > In the context of Docker, the Docker Server is effectively just an API. 

    **Answer:** API

## Task 7 - Practical
1. Connect to the machine. What is the name of the container that is currently running?<br>
    Use `docker ps` to list current running container.<br>
    **Answer:** cloudisland

1. What is the flag?<br>
    Use `docker ps -a` to list all the containers, then start with `docker start <id>`.
    Access to the given link, you will receive the flag.<br>
    **Answer:** THM{WEBSERVER_CONTAINER}
