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
