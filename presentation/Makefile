presentation:
	pandoc -s presentation.md \
	-t beamer \
	-o presentation.pdf \
	--filter pandoc-latex-fontsize \
	--slide-level 3 \
	--columns=72 \
	--wrap=auto \
	-f markdown+implicit_figures \
	-f markdown+hard_line_breaks
