#lang racket
; Part IV: Scheme Preparation

; Names for grabbing items from a list
(define first (lambda (x) (car x)))
(define second (lambda (x) (cadr x)))
(define rest (lambda (x) (cdr x)))
(define third (lambda (x) 'todo))
(define fourth (lambda (x) 'todo))
(define fifth (lambda (x) 'todo))

; Counts how many #t values are in the list
(define (truecount lst) 'todo)

; Squares every item in the list
(define (squarelist lst) 'todo)

; Keeps only the numbers bigger than 100
(define (hundreds? lst) 'todo)

; Prints the sequence until it reaches 1
(define (collatz n) 'todo)
