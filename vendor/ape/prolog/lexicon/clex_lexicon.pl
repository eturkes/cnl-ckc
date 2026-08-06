% This file is part of the Attempto Parsing Engine (APE).
% Copyright 2008-2013, Attempto Group, University of Zurich (see http://attempto.ifi.uzh.ch).
%
% The Attempto Parsing Engine (APE) is free software: you can redistribute it and/or modify it
% under the terms of the GNU Lesser General Public License as published by the Free Software
% Foundation, either version 3 of the License, or (at your option) any later version.
%
% The Attempto Parsing Engine (APE) is distributed in the hope that it will be useful, but WITHOUT
% ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
% PURPOSE. See the GNU Lesser General Public License for more details.
%
% You should have received a copy of the GNU Lesser General Public License along with the Attempto
% Parsing Engine (APE). If not, see http://www.gnu.org/licenses/.
%
% Modified 2026-08-06 for this fork: the ~2000-entry demonstration common
% lexicon is replaced by the minimal entry set exercised by tests/red/;
% guideline vocabulary lives in per-guideline lexicon.ulex files.
% Empty lexical predicates stay declared dynamic in clex.pl, so lookups
% on absent words fail (unknown word) instead of raising.

pn_sg('John', 'John', masc).
pn_sg('Mary', 'Mary', fem).

noun_sg(man, man, masc).

iv_finsg(sleeps, sleep).
iv_finsg(works, work).
iv_infpl(sleep, sleep).
iv_infpl(work, work).

tv_finsg(sees, see).
