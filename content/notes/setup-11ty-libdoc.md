---
title: Setup LibDoc
description: This page describes the steps I followed to set up this 11ty/Libdoc site 
layout: libdoc_page
language: en
permalink: /notes/setup-11ty-libdoc/index.html
eleventyNavigation:
    parent: Latest pages
    key: Setup LibDoc
tags:
    - LibDoc
    - Eleventy
date: 2026-01-08
---

# Setup 11ty LibDoc


## Useful links

https://www.11ty.dev/docs/starter/

personal knowledge base eleventy
https://eleventy-libdoc.netlify.app/#getting-started
 
https://eleventy-excellent.netlify.app/blog/eleventy-excellent-40/

## Motivation


## Procedure

```bash
git clone https://github.com/ita-design-system/eleventy-libdoc knowledge-base
cd knowledge-base
git branch -M main
git remote set-url origin git@github.com:Jallalbrahimi/knowledge-base.git
git push -u origin main
```

```bash
deno install
deno approve-scripts npm:sharp@0.33.5
dx @11ty/eleventy  # build
dx @11ty/eleventy --serve # run locally
```

## Configure DNS

Add a new CNAME entry :

```
The CNAME record currently generated is as follows:
kb IN CNAME jallalbrahimi.github.io.
```

in the repo, add a CNAME file that contains the custom domain

Github will figure out the right subdomain to use using the 

## Configure Github Pages

Go to the Pages configuration in Settings https://github.com/Jallalbrahimi/knowledge-base/settings/pages

Source: GitHub Actions
Custom domain: kb.jallalbrahimi.com
