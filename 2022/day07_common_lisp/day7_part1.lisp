#!/usr/bin/gcl -f


(defun read-ls-entry ()
    (let ((symbol-1 (read *standard-input* nil)))
        (if (member symbol-1 '(nil $))
            0
            (let ((name (read)))
                (+
                    (if (equal symbol-1 'dir)
                        0         ; directory entry doesn't really mean anything to us here
                        (block file
                            (format t "size of ~a = ~d" name symbol-1)
                            (terpri)
                            symbol-1
                        )
                    )
                    (read-ls-entry)
                )
            )
        )
    )
)


(defun explore-dir (name)
    (format t "dir ~a" name)
    (terpri)
    
    (let ((total-size 0) (sum-of-small-dirs 0))
        (loop
            (setq command (read *standard-input* nil))
            (when (equal command '$) (setq command (read *standard-input* nil)))
            (when (equal command nil) (return))
            (if (equal command 'ls)
            
                (block ls
                    (setq total-size (+ total-size (read-ls-entry)))
                    (format t "files: ~d" total-size)
                    (terpri)
                )
                
                (let ((sub-dir (read-line)))
                    (when (equal sub-dir "..") (return))
                    (multiple-value-bind (sub-size sub-sum) (explore-dir sub-dir)
                        (setq total-size (+ total-size sub-size))
                        (setq sum-of-small-dirs (+ sum-of-small-dirs sub-sum))
                    )
                )
            )
        )
        
        (when (<= total-size 100000)
            (setq sum-of-small-dirs (+ sum-of-small-dirs total-size))
        )
    
        (format t "dir ~a: total-size=~d, sum-of-small-dirs=~d" name total-size sum-of-small-dirs)
        (terpri)
        
        (values total-size sum-of-small-dirs)
    )
)

(read-line)
(multiple-value-bind (_ size) (explore-dir "/")
    (format t "~d" size)
    (terpri)
)
