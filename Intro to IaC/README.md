> # Intro to IaC

# Summary
- [Summary](#summary)
  - [Task 2 - IaC - The Concept](#task-2---iac---the-concept)
  - [Task 3 - IaC - The Tools Part 1](#task-3---iac---the-tools-part-1)
  - [Task 4 - IaC - The Tools Part 2](#task-4---iac---the-tools-part-2)
  - [Task 5 - Infrastructure as Code Lifecycle](#task-5---infrastructure-as-code-lifecycle)
  - [Task 6 - Virtualisation \& IaC](#task-6---virtualisation--iac)
  - [Task 7 - On-Prem IaC vs. Cloud-Based IaC](#task-7---on-prem-iac-vs-cloud-based-iac)
  - [Task 8 - IaC - The Final Push](#task-8---iac---the-final-push)

##  Task 2 - IaC - The Concept
1. Your organisation is preparing to launch a new service called FlyNet. The DevSecOps team provisioned an infrastructure and tested this service in the dev environment. Which IaC characteristic will streamline the provisioning of this same infrastructure in staging and production?<br>
    > Developers first build their code in a dev environment before testing it in a staging environment (which mirrors production) and finally deploying the fully functional and tested code into the production environment.

    **Answer:** Repeatable 

1. It's the day before launch, and the latest infra change has started producing strange errors, something about "destroying humanity". Weird! Which IaC characteristic allows us to go back to the last known working version?<br>
    > Speaking of infrastructure issues, versioning also means that should your latest infrastructure start having problems, you can redeploy your infrastructure to the last known working version

    **Answer:** Versionable

1. It's launch day, and it couldn't have gone better; the service is almost running itself! The FlyNet launch has attracted a lot of new customers! Which IaC characteristic enables us to increase the resources available to our infrastructure to meet this increased demand with ease?<br>
    > This technology begins to make even more sense when you consider the widespread adoption of cloud computing and its elastic nature, with virtual machines and resources being scaled up and down based on demand due to resources being billed per second/minute/hour. 

    **Answer:** Scalable

##  Task 3 - IaC - The Tools Part 1
1. In the scenario given, which type of IaC tool considers where you are on the map and gives instructions to reach the desired X point?<br>
    **Answer:** Declarative

##  Task 4 - IaC - The Tools Part 2
1. Can you retrieve the location and retrieve the flag?
    Just follow the instruction and answer all the question to get the flag.<br>
    **Answer:** THM{l4b_C0mpl3x_co0rds}

## Task 5 - Infrastructure as Code Lifecycle
1. A DevSecOps Engineer at CyberMyne is looking for guidance on developing their next infrastructure. What type of phases provide guidance during the development or configuration of an infrastructure?<br>
    > Repeatable(Infra Creation + Config) Phases: Repeatable phase block These phases are done during the creation/configuration of an infrastructure and are done one or many times at different points and with differing variations depending on what needs to be done.

    **Answer:** Repeatable

1. What type of phases ensure best practices throughout infrastructure development and management?<br>
    > Continual (Best Practice) Phases: Continual phase block These phases are continually done to ensure best practices throughout the various stages of infrastructure development and management. 

    **Answer:** Continual 

1. The 'Monitoring/Maintenance' continual phase can trigger which other continual phase?
    > Monitoring/Maintenance: Once an infrastructure has been provisioned/configured, it must be monitored for poor performance, security events, failure events, warnings, etc. Automated maintenance tasks (e.g. disk clean-up) can help avoid some of these events. In certain events, this phase can trigger the rollback phase.

    **Answer:** Rollback

##  Task 6 - Virtualisation & IaC
1. CyberMine is deploying the latest machine model E-1000. This model requires virtualisation at an operating system level to allow for lightweight and rapid deployment behind the scenes! What level of virtualisation would be needed for this?<br>
    > Containerisation (platform example: Docker) is virtualisation at an operating system level, meaning the same operating system kernel is used to run multiple containers.

    **Answer:** Containerisation 

1. CyberMine's E-100 Model is still very popular for all your extermination needs, this model requires multiple OS to run on a single machine. Which level of virtualisation would be needed for this?<br>
    > Hypervisor virtualisation (e.g. VMware) enables multiple virtual machines to run on a single physical server. Imagine a physical server in a data centre.

    **Answer:** Hypervisor 

1. The new E-1000 model has a feature that allows it to pass through physical objects. Wild! This new feature, however, is very resource-intensive. Which 'Use of IaC' will ensure that this resource consumption won't affect the performance of the machine's other components?<br>
    > Resource Isolation: Virtualisation enabled resource isolation between different VMs/containers. Using IaC, you can define what components get how much of the available resources. This means if one component starts getting hammered (consuming all of its allocated CPU, memory, bandwidth, etc.), it won’t affect the performance of the other components.

    **Answer:** Resource Isolation

1. Due to the resource consumption of this new feature, it requires rapid scaling of resources. Which container orchestration software can be used to automate this process?<br>
    **Answer:** Kubernetes

##  Task 7 - On-Prem IaC vs. Cloud-Based IaC
1. Cloud-based resources are provisioned/configured in a cloud environment. Who handles the underlying infrastructure?<br>
    > Cloud-based: This means provisioning and configuring infrastructure resources in a cloud environment using cloud resources. This is done using a cloud service provider (CSP) such as AWS, Microsoft Azure, GCP, etc.

    **Answer:** cloud service provider

1. What category does on-prem infrastructure struggle with due to hardware limitations when facing increased traffic?<br>
    > On-prem: Scaling resources using on-prem IaC can be a slow and cumbersome process. This can involve manual/physical changes and procurement. Because scalability is such a slow process, consideration is required, and hardware limitations must be enough to handle peak load. This can prove challenging in situations where an on-premises infrastructure starts receiving increased traffic (think enrolment or exam season at a university/college).

    **Answer:** scalability

##  Task 8 - IaC - The Final Push
1. Can you get the flag using your infrastructure as code skills?<br>
    Win 3 level to get the flag.<br>
    **Answer:** thm{1Nfr4StrUctUr3_Pr0}
