#!/bin/bash

echo "Replace pattern:"
echo "$(printf '${%s} ' $(env | cut -d'=' -f1))"

for d in /etc/nginx/conf.d/*; do
  if [ -d "$d" ]; then
    for file in "$d"/*; do
      file_name=$(basename $file)
      if [ ! -d "$file" ] && echo "$file_name" | grep -q ".template"; then
        out_dir=$(echo $file | sed 's/.template//')

        echo "Building subconfig from: $file"

        envsubst "$(printf '${%s} ' $(env | cut -d'=' -f1))" <"$file" >"$out_dir"

        echo "Emitted: $out_dir"
      fi
    done
  else
    file_name=$(basename $d)
    if echo "$file_name" | grep -q ".template"; then
      out_dir=$(echo $d | sed 's/.template//')
      echo "Building config from: $d"

      envsubst "$(printf '${%s} ' $(env | cut -d'=' -f1))" <"$d" >"$out_dir"

      echo "Emitted: $out_dir"
    fi
  fi
done

echo "------------- NGINX WORKING DIRECTORY ------------"
find /etc/nginx -print | sed -e "s/\/[^\/]*\//  |/g" -e "s/|\([^ ]\)/| - \1/"
echo "--------------------------------------------------"

nginx -g "daemon off;"
