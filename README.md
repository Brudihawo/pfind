# pfind

Query pdfs in a directory via TF-IDF.

```console
Usage: pfind [OPTIONS] <QUERY>

Arguments:
  <QUERY>  

Options:
  -d, --directory <DIRECTORY>      Directory to process [default: .]
  -p, --page-thresh <PAGE_THRESH>  Page number threshold for PDFs to ignore [default: 25]
  -v, --verbose                    Verbose output
  -h, --help                       Print help
  -V, --version                    Print version
```

# Roadmap
- [ ] optional gui for repeated queries
- [ ] possibly cache results of indexing
- [ ] better search metrics than tf-idf
- [ ] stemming
