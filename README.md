# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. Based on my understanding of Observer design patterns, a single model stuct is enough because Rust's ownership model ensures that data is accessed and modified in a controlled manner, reducing the need for interfaces. However, interfaces can provide a separation of concerns and create more modular and maintainable code.

2. Based on my understanding of Vec and DashMap, DashMap guarantees uniqueness, has a faster retrieval time, enables concurrency and dynamic updates. The additional logic provided for DashMap to ensure these benefits is not provided for Vec. Therefore, DashMap should be implemented instead of Vec.

3. Based on my understanding design patterns, while the Singleton pattern provides a single access point to the SUBSCRIBERS list, ensuring thread safety would require additional complexity and potential performance overhead because of mutex locking or atomic operations. DashMap is designed for concurrent access and handles concurrency internally, making it a better choice for ensuring thread safety. Therefore, we still need DashMap.

#### Reflection Publisher-2
1. We need to separate "service" and "repository" from a "model" because it allows us to have cleaner code, and reduces future overhead by improving maintainability and testability. This is because it promotes flexibile and reusabile code. Futhermore, Separation of Concerns is a primary design principle.

2. If we only use the "model" without separating concerns into "service" and "repository" layers, it would result in a complex codebase that would be hard to maintain. It would it difficult to adapt the system to changing requirements or to reuse components in different contexts.

3. I have not made the time to further explore Postman. However, my experience with the application made it easier to test https requests from my application. I intend to use it in my workflow for future projects.

#### Reflection Publisher-3
1. The push model is used as the observer pattern, where the NotificationService acts as the publisher pushing notification data to subscribers without requiring them to actively request it. This simplifies the implementation and allows for easy communication between the publisher and subscribers.

2. The pros and cons of using the other variation of observer pattern for this tutorial would be as follows. Using the pull model variation, subscribers could have more control over when they receive updates and fetch data. Furthermore, pulling updates only when necessary would reduce network traffic. However, the pull model could introduce overhead, latency, and increased coupling between subscribers and publishers.

3. By not using multi-threading in the notification process, it can result in blocking behavior, decreased performance, reduced responsiveness, limited concurrency, scalability challenges, and potential synchronization issues. Multi-threading is important for improving performance, concurrency, and responsiveness in applications that involve asynchronous tasks, for example, sending notifications to subscribers in real-time systems.