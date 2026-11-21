#lang racket
; Part IV: Scheme Preparation

;Names for grabbing items from a list
(define first (lambda (x) (car x)))
(define second (lambda (x) (cadr x)))
(define rest (lambda (x) (cdr x)))
; third is the car of the cdr of the cdr
(define third (lambda (x) (caddr x)))
; fourth skips three items and takes the next one
(define fourth (lambda (x) (cadddr x)))
;fifth skips four items and takes the next one
(define fifth (lambda (x) (car (cddddr x))))

;Counts how many #t values are in the list
(define (truecount lst)
  (cond
    ; nothing left to count
    ((null? lst) 0)
    ;The front item is #t so count it and recurse on the rest
    ((eq? (car lst) #t) (+ 1 (truecount (cdr lst))))
    ; not #t so skip it and keep counting the rest
    (else (truecount (cdr lst)))))

; Squares every item in the list using map and a lambda
(define (squarelist lst)
  (map (lambda (x) (* x x)) lst))

;Keeps only the numbers bigger than 100 using filter and a lambda
(define (hundreds? lst)
  (filter (lambda (x) (> x 100)) lst))

; Prints the sequence until it reaches 1
(define (collatz n)
  (cond
    ;The sequence only works on positive numbers
    ((< n 1) (display "n has to be a positive integer") (newline))
    ; Reached 1 so print it and stop
    ((eq? n 1) (display n) (newline))
    ; n is even so print it and keep going with n / 2
    ((even? n) (display n) (newline) (collatz (/ n 2)))
    ;n is odd so print it and keep going with 3n + 1
    (else (display n) (newline) (collatz (+ (* 3 n) 1)))))

;examples from the project doc to test first through fifth
(define family '(josh sara erin sandy jon))
(first family)
(second family)
(third family)
(fourth family)
(fifth family)

; examples to test truecount, squarelist and hundreds?
(truecount '(#t #f #t #t #f))
(squarelist '(1 2 3 4))
(hundreds? '(50 101 250 100 99 1000))

;examples to test collatz
(collatz 6)
(collatz 7)
