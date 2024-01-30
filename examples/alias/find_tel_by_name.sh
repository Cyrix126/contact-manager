#!/bin/bash

fullname=$1

cm find-value -f 'FN:'$1'' -s TEL --forgive --pretty
