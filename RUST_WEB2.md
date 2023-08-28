
# Rust Web 2.0 Development

## Frameworks

> **Actix** is a actor framework.

> **Tokio** is used as a runtime which is the asynchronous engine behind Actix.

## ORMs

> **Diesel** does not support tokio(It means Actix does not support cos Actix is based on *actor model ref.glossary*), so we have to run it in separate threads using the **web::block** function, which offloads blocking code (like Diesel's) to do not block the server's thread.

> **Rocket** is an ORM-agnostic.

## Dependant Libraries

- [x] Chrono: Timestamp, DateTime
- [x] r2d2: Connection pool
- [x] Migration_tools: Sqlx migrate


# Glossary

## Backpressure

Backpressure is a pattern for designing applications that respond well to high load. For example, the mpsc channel comes in both a bounded and unbounded form. By using the bounded channel, the receiver can put "backpressure" on the sender if the receiver can't keep up with the number of messages, which avoids memory usage growing *without bound as more and more messages are sent on the channel.*

## Actor

A design pattern for designing applications. **An actor refers to an independently spawned task** *that manages some resource on behalf of other parts of the application*, **using channels to communicate** *with those other parts of the application.*
