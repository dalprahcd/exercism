(import (rnrs))

(define (leap-year? year)
  (cond
    [(zero? (modulo year 400)) #t]
    [(zero? (modulo year 100)) #f]
    [(zero? (modulo year 4)) #t]
    [else #f]))
