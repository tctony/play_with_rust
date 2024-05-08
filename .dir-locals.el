;;; Directory Local Variables            -*- no-byte-compile: t -*-
;;; For more information see (info "(emacs) Directory Variables")

(
 (rust-mode . (
               (quickrun-option-cmdkey . "cargo-run")
               (eval . (setq-local
                       quickrun-option-args (file-name-nondirectory (file-name-sans-extension (buffer-file-name)))
                       ))
               ))
 )
