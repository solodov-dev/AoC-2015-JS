BEGIN { FS="" }
{
	pair = repeat = 0;
	for (i = 1; i < NF - 1; i++) {
		if (!pair && substr($0, i + 2) ~ $i $(i + 1))
			pair = 1
		if (!repeat && $i == $(i + 2))
			repeat = 1
		if (pair && repeat) {
			good++
			next
		}
	}
}
END { print good }
