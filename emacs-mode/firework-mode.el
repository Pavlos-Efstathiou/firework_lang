;;; firework-mode.el --- Description -*- lexical-binding: t; -*-
;;
;; Copyright (C) 2021 Pavlos Efstathiou
;;
;; Author: Pavlos Efstathiou <https://github.com/pavlos>
;; Maintainer: Pavlos Efstathiou <pavlosefstathiou2009@gmail.com>
;; Created: November 17, 2021
;; Modified: November 17, 2021
;; Version: 0.0.1
;; Keywords: abbrev bib c calendar comm convenience data docs emulations extensions faces files frames games hardware help hypermedia i18n internal languages lisp local maint mail matching mouse multimedia news outlines processes terminals tex tools unix vc wp
;; Homepage: https://github.com/pavlos/firework-mode
;;
;; This file is not part of GNU Emacs.
;;
;;; Commentary:
;;
;;  Description
;;
;;; Code:

(setq firework-font-lock-keywords
      (let* (
             (x-keywords '("let" "import" "module" "enum" "if" "else"))
             ;; TODO Haskell types and functions
             (x-types '("Int" "Integer" "String" "Char" "IO" "Bool"))
             (x-comments '("/*" "*/"))

             (x-comments-regexp (regexp-opt x-comments 'comment))
             (x-keywords-regexp (regexp-opt x-keywords 'words))
             (x-types-regexp (regexp-opt x-types 'words)))

        `(
          (,x-types-regexp . 'font-lock-type-face)
          (,x-keywords-regexp . 'font-lock-keyword-face)
          (,x-comments-regexp . 'font-lock-comment-face))))

(defun firework-mode-find-main
    ()
    "Used to find the definition of the main function in a Firework program."
    (interactive)
    (when (re-search-forward "let\s+main" nil t 1)
      (setf
       (point) (match-beginning 0)
       (mark) (match-end 0))))

(defun firework-mode-run
  ()
  "Run a project."
  (interactive)
  ;; TODO: Find the project root directory and run the project from there
  (shell-command "firework_lang run"))

(defvar firework-mode-map
  (let ((map (make-sparse-keymap)))
    (define-key map (kbd "SPC f m") 'firework-mode-find-main)
    map))

(define-derived-mode firework-mode
  special-mode "Firework Mode"
  "Major mode for editing Firework"
  (setq font-lock-defaults '(firework-font-lock-keywords)))

(provide 'firework-mode)
;;; firework-mode.el ends here
