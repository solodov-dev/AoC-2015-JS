BEGIN { FS="" }
/^(.*[aeiou].*){3,}$/ && $0 !~ /ab|cd|pq|xy/ { 
  for (i = 1; i < NF; i++) {
		if ($i == $(i + 1)) {
			good++; next
		}
	}
}
END { print good }
