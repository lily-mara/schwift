# The Schwift Programming Language

![Schwift logo](logo.png)

[![Build Status](https://travis-ci.org/natemara/schwift.svg?branch=master)](https://travis-ci.org/natemara/schwift)

Schwift is an imperative programming language based on the fantastic show, Rick and Morty. It supports all of the classic language features required to elegantly build fantastic programs.

## Variables

Schwift is a dynamically typed language:

```schwift
>>> x squanch 10
>>> show me what you got x
10

>>> x squanch "Hello"
>>> show me what you got x
Hello
```

## Lists

Schwift supports dynamically typed lists as a first-class type:

```schwift
>>> x on a cob
>>> x assimilate 10
>>> x assimilate "hello"
>>> show me what you got x
[Int(10), Str("hello")]
```

## Memory management

Schwift has manual memory management through the flexable `squanch` keyword:

```schwift
>>> x squanch 10
>>> squanch 10
>>> show me what you got x
error: x is undefined
```
