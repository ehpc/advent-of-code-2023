#lang racket

(define input-file "input1.txt")
(define num-rx #rx"([0-9]|one|two|three|four|five|six|seven|eight|nine)")

(define (word-to-digit word)
  (match word
    ["one" "1"]
    ["two" "2"]
    ["three" "3"]
    ["four" "4"]
    ["five" "5"]
    ["six" "6"]
    ["seven" "7"]
    ["eight" "8"]
    ["nine" "9"]
    [_ word]))

(define (get-list-of-digits line)
  (map word-to-digit (regexp-match* num-rx line)))

(define (concat-two-or-one-digits line)
  (let ([digits (get-list-of-digits line)])
    (if (= 1 (length digits))
      (string-append (car digits) (car digits))
      (string-append (car digits) (last digits)))))

(define (parse-input port) 
  (for/sum ([line (in-lines port)])
    (string->number (concat-two-or-one-digits line))))

(call-with-input-file input-file parse-input)
