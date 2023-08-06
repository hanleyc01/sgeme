(export main)
(define (main args)
  (display "fart\n"))

;; after core form translation
(define main 
  (lambda (args)
    (display "fart\n")))