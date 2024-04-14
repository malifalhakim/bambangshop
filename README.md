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
1. Pada *Observer design patterns* akan lebih baik jika kita menggunakan interface (atau traits). Hal ini dikarenakan kita akan mendapatkan fleksibilitas jika sekiranya kita ingin menambahkan tipe subscriber yang berbeda. Dengan adanya interface kita tidak perlu melakukan modifikasi atau perubahan pada code yang telah kita buat, karena kita telah menerapkan *Open-close principle*. Akan tetapi, jika hanya terdapat satu tipe subscriber dan dapat dipastikan tidak akan bertambah, maka penggunaan *single Model struct* sudah cukup. Oleh karena itu, jika pada kasus BambangShop dapat dipastikan hanya terdapat satu tipe subscriber, maka penggunaan *single model struct* sudah cukup.

2. Menurut saya, penggunaan DashMap diperlukan untuk kasus ini. Hal ini dikarenakan dengan dengan DashMap kita dapat menyimpan key-value *pairs* dikarenakan *key*-nya yang unik. DashMap dapat membantu kita dengan lebih mudah melakukan pencarian dengan *key* tersebut. Pada kasus ini, dengan DashMap kita dapat dengan mudah mencari Subscriber berdasarkan url-nya. Sementara itu, jika kita menggunakan Vec, maka kita mungkin perlu melakukan iterasi untuk menemukan Subscriber yang diinginkan sehingga hal ini dapat mempersulit proses pengambilan dan pengupdate-an data.

3. Penggunaan DashMap pada Singleton pattern membantu aplikasi atau program untuk menangani *reads* dan *writes* secara bersamaan. Oleh karena itu, akan lebih baik jika kita menggunakan DashMap dibanding HashMap. Hal ini dikarenakan DashMap lebih cocok untuk aplikasi multithreading dimana aplikasi yang kita buat memerlukan hal ini karena akan diakses oleh banyak thread. Dengan singleton pattern seperti yang telah kita implementasikan, kita dapat memastikan objek-objek subscriber hanya berada pada 1 DashMap karena singleton pattern memastikan hanya terdapat 1 instance dari objek tersebut.

#### Reflection Publisher-2
1. Pemisahan *Repository* dan *Service* perlu dipisahkan dari *Model* bertujuan untuk memenuhi *Single Responsibility Principle* dimana setiap objek seharusnya hanya melakukan atau bertanggung jawab pada satu hal saja. Pada MVC, model mengetahui banyak hal mulai dari *bussiness logic*, validasi, data itu sendiri, dan operasi penyimpanan data. Dengan memisahkan *bussiness logic* ke *Service* dan *data storage operations* ke *Repository*, sekarang *Model* hanya berfungsi sebagai representasi atau struktur dari sekumpulan data

2. Jika kita hanya menggunakan *Model*, maka kode untuk setiap *Model* akan menjadi besar. Hal ini akan mengakibatkan kode semakin kompleks dan sulit untuk dipahami. Selain itu, hal ini juga akan menyebabkan pelanggaran *Single Responsibility Principle* sehingga kode menjadi sulit untuk di-*maintain*. Contohnya jika kita ingin mengubah suatu *logic*, kita perlu untuk memeriksa apakah perubahan tersebut berdampak pada bagian lain. Ditambah jika kita tidak melakukan pemisahan, maka dapat menghasilkan program dengan tingkat *coupling* yang tinggi.

3. Menurut saya, Postman adalah tool yang sangat bermanfaat untuk membuat suatu aplikasi terutama dalam mengetes API. Postman dapat membantu saya dalam mengirim HTTP request dan membantu saya dalam memeriksa *response*-nya juga. Hal ini sangat membantu saya dalam melakukan *debugging* ataupun meningkatkan kualitas API.


#### Reflection Publisher-3
1. Pada kasus ini, kita menggunakan variasi *Push Model*. Hal ini dikarenakan setiap terjadi penambahan atau penghapusan product, method `notify` pada `NotificationService` dieksekusi. Hal ini menggambarkan *publisher* mempunyai kemampuan untuk membuat keputusan untuk mengirim notifikasi dan data pada notifikasi kepada *subscriber*-nya. *Subscriber* pun menerima notifikasi tersebut tanpa perlu melakukan *request*. 

2. Jika kita menggunakan *pull model*, maka *subscriber* yang menentukan kapan mereka akan mengambil data atau menerima notifikasi dari *publisher*. Keuntungannya pada kasus ini adalah *subscriber* dapat menentukan kapan mereka akan mengambil data dari *publisher* sehingga mereka tidak terganggu dengan notifikasi pada saat yang tidak diinginkan. Selain itu, dengan model ini, *subscriber* juga dapat menentukan data apa yang akan mereka ambil. Kelemahannya adalah *subscriber* tidak dapat langsung/dengan segera menerima data atau  notifikasi jika terjadi penambahan atau penghapusan product.

3. Jika kita tidak menggunakan *multi-threading*, maka aplikasi akan mengirim notifikasi secara satu persatu. Hal ini dapat memperlambat proses pengiriman notifikasi terutama jika terdapat banyak *subscriber*. Jadi program tidak akan menjadi error, tetapi hal ini dapat memperlambat proses notifikasi secara signifikan.