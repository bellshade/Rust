# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html
import sys, os



# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information



project = 'Rust-docs'
copyright = '2023, mukharomdev'
author = 'mukharomdev'
release = '0.0.1'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

source_suffix = '.rst'
extensions = []
# The master toctree document.
master_doc = 'index'
templates_path = ['_templates']
exclude_patterns = []
pygments_style = 'sphinx'
highlight_language = 'rust'
html_title = 'Pengenalan Rust Bahasa Indonesia'

language = 'id'

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'haiku'
html_static_path = ['_static']
