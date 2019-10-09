#!/bin/bash
if [ "$1" == "clean" ];then
    make clean
else
    swipl -g 'use_module(library(prolog_pack))' -g 'prolog_pack:save_build_environment('./').' -g halt.
    source ./buildenv.sh
    make $*
fi
