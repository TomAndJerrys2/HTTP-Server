# Multi-Threaded HTTP Server

This is a Simple HTTP Server Built using Rust for its proficiency in speed and
ease of multi-threaded programming. This was a great project for seeing the full
picture into the HTTP Protocol and how servers like apache may work

Initially, This was meant to be just a HTTP Server but with my usual excitement,
I began plotting more features and expanding on the basis of the idea. Implementing
Custom server responses and request makes the server robust and less prone to malignant
errors or bad requests. Rust shined in this aspect where by it was much easier to catch
errors and play it safe - One of the things that was most important was the content buffer
size being dynamic allowing requests of differing sizes to be processed through the server

# Features
- Freestanding server binary using cargo
- Request and Response resolve handlers
- CRUD Operations via HTTP Requests (upload, download etc)
- Local Web Storage (Local Caching)
- Request and Response Modifiers
- HTTPS Coming Soon! :)

# Further Implementation Ideas
For further implementation (as with most projects there are features to be added :) )
it may be a good idea to introduce SSL as a layer of security to the web server and
the clients accessing it. Additionally, HTTPS would be preferred as the standard over
HTTP in terms of application security. One last implemenation idea would be for the
speed of the application - While the server already takes advantage of Multi threading
aspects, adding a local cache may improve performance drastically

# Benchmark Results -> Optimizations
For benchmarking and the nature of the project I thought it suitable to use Google's
benchmarking library for testing the speed of both the web server req/res times and caching
