# coding: utf-8
from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
  name="skim-unicode-table",
  # version is handled dynamically by setuptools_scm
  use_scm_version = True,
  description="Interactive unicode table using lotabout's skim",
  keywords="",
  url="",
  author="Shahriar Heidrich",
  author_email="smheidrich@weltenfunktion.de",
  classifiers=[
    "Development Status :: 3 - Alpha",
    "Environment :: Console",
    "Operating System :: POSIX :: Linux",
    "Programming Language :: Python :: 3.6",
  ],
  packages=["skim_unicode_table"],
  rust_extensions=[
    RustExtension({"sk": "skim_unicode_table.sk"},
      path="skim-fork/Cargo.toml", binding=Binding.Exec),
    RustExtension(
      {"print-unicode-table": "skim_unicode_table.print-unicode-table"},
      path="print-unicode-table/Cargo.toml", binding=Binding.Exec)
  ],
  package_data={
    "skim_unicode_table": ["*.sh", "render-glyph"]
  },
  # Rust binary definitely not zip safe, neither are shell scripts referring to
  # each other
  zip_safe=False,
  scripts=[
    "scripts/skim-unicode-table",
    "scripts/skim-unicode-table-xsel",
  ],
  setup_requires=[
    "pytest-runner",
    "setuptools_scm",
  ],
  install_requires=[
    "cairocffi",
    "pangocairocffi",
  ],
  tests_require=[
    "pytest",
  ],
)
