#lang racket
;Part V: Scheme Math Translation

; Number words in Chinese and English
(define chinese '(ling yi er san si wu liu qi ba jiu shi))
(define english '(zero one two three four five six seven eight nine ten))

;finds where the word is in the list and gives back its spot as the number
(define (find-index word lst index)
  (cond
    ; the front word matches so this spot is the number
    ((eq? (car lst) word) index)
    ; not a match so keep checking the rest and move the spot up one
    (else (find-index word (cdr lst) (+ index 1)))))

; Turns a Chinese or English word into its number
(define (translate-word word)
  (cond
    ; It was a Chinese word so use that number
    ((member word chinese) (find-index word chinese 0))
    ;It was an English word so use that number
    ((member word english) (find-index word english 0))))

; Checks if the word is in either list
(define (known? word)
  (if (or (member word chinese) (member word english)) #t #f))

;Drops the words it doesn't know and translates the rest
(define (translate-all lst)
  (map translate-word (filter known? lst)))

; Prints the numbers with the separator between each one
(define (show-list nums sep)
  (cond
    ; Nothing left to print
    ((null? nums) (void))
    ; Last number so no separator after it
    ((null? (cdr nums)) (display (car nums)))
    ;Print the front number and separator and recurse on the rest
    (else (display (car nums)) (display sep) (show-list (cdr nums) sep))))

;Translates the words, then shows the sum and product
(define (go lst)
  (let ((nums (translate-all lst)))
    (display "Translation: ") (show-list nums " ") (newline)
    (display "Addition: ") (show-list nums " + ")
    (display " = ") (display (apply + nums)) (newline)
    (display "Multiplication: ") (show-list nums " * ")
    (display " = ") (display (apply * nums)) (newline)))

; examples from the project doc to test
(go '(yi nine six ba))
(newline)
;josh should get filtered out
(go '(yi josh three si))
