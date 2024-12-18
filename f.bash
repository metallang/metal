shopt -s extglob

RED='\033[0;31m' # red color
NC='\033[0m' # no color aka color reset
EXCLUDE_PATTERN='+(LICENSE|*.toml|mir-tests/*)'
HEADER='SPDX-License-Identifier: MIT'
FAILURE=false
FILES=$(git ls-files)

for filename in $FILES; do
  if [[ "$filename" == $EXCLUDE_PATTERN ]]; then
    echo "Skipping $filename because it was manually exempted from the check"
    continue
  fi

  FILE_HEAD=$(head $filename)

  # grep returns 1 (failure) if it doesn't find a match
  if ! (echo "$FILE_HEAD" | grep -q "$HEADER"); then
    echo -e "${RED}File $filename does not have an SPDX license header${NC}"
    FAILURE=true
  fi
done

if [ "$FAILURE" = true ]; then
  exit 1
fi
