#!/usr/bin/env swipl

%%
%% Print the value of the SWIARCH environment variable.
%%

:- current_prolog_flag(arch, Arch),
   format(Arch),
   halt.
