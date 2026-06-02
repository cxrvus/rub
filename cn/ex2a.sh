sort grades.csv | uniq -d | cut -d , -f 1 | sha1sum
