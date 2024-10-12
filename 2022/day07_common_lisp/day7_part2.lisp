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
    
    (let ((total-size 0) (dir-size-list (list)))
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
                    (multiple-value-bind (sub-size sub-list) (explore-dir sub-dir)
                        (setq total-size (+ total-size sub-size))
                        (setq dir-size-list (append dir-size-list sub-list))
                    )
                )
            )
        )
        
        (setq dir-size-list (append dir-size-list (list total-size)))
        
        (format t "dir ~a: total-size=~d, dir-size-list=~d" name total-size dir-size-list)
        (terpri)
        
        (values total-size dir-size-list)
    )
)

(defun find-required-dir (required-size sorted-dir-list)
    (let ((first (car sorted-dir-list)))
        (format t "required-size=~d, dir size=~d" required-size first)
        (terpri)
        (if (>= first required-size)
            first
            (find-required-dir required-size (cdr sorted-dir-list))
        )
    )
)

(read-line)
(multiple-value-bind (total-size dir-sizes) (explore-dir "/")
    (format t "~d" (find-required-dir (- total-size 40000000) (sort dir-sizes #'<) ))
    (terpri)
)
