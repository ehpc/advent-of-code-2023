#lang racket

(define input-file "input1.txt")

(define (get-list-of-digits line) (regexp-match* #rx"[0-9]" line))

(define (concat-two-or-one-digits line)
  (let ([digits (get-list-of-digits line)])
    (if (= 1 (length digits))
      (string-append (car digits) (car digits))
      (string-append (car digits) (last digits)))))

(define (parse-input port) 
  (for/sum ([line (in-lines port)])
    (string->number (concat-two-or-one-digits line))))

(call-with-input-file input-file parse-input)
