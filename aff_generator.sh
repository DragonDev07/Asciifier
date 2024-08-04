# Check if the font name is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <figlet-font-name>"
  exit 1
fi

# Get the font name from the argument
FONT_NAME=$1

# Create File
touch "$FONT_NAME".aff

write_ascii()
{
  local input_string="$1"
  echo "$1" >> "$FONT_NAME.aff"
  figlet -f $FONT_NAME $1 >> $FONT_NAME.aff
  echo "" >> $FONT_NAME.aff
}

# Uppercase Letters
for letter in {A..Z}; do
  write_ascii "$letter"
done

# Lowercase Letters
for letter in {a..z}; do
  write_ascii "$letter"
done

# Characters
write_ascii "~"
write_ascii "!"
write_ascii "@"
write_ascii "#"
write_ascii "$"
write_ascii "%"
write_ascii "^"
write_ascii "&"
write_ascii "*"
write_ascii "("
write_ascii ")"
write_ascii "_"
write_ascii "+"
write_ascii "|"
write_ascii "-"
write_ascii "="
write_ascii "{"
write_ascii "}"
write_ascii "["
write_ascii "]"
write_ascii ":"
write_ascii ";"
write_ascii "'"
write_ascii "\""
write_ascii ","
write_ascii "."
write_ascii "<"
write_ascii ">"
write_ascii "?"
write_ascii "/"
write_ascii "\\"
write_ascii "\`"
write_ascii "1"
write_ascii "2"
write_ascii "3"
write_ascii "4"
write_ascii "5"
write_ascii "6"
write_ascii "7"
write_ascii "8"
write_ascii "9"
write_ascii "0"

echo "--- Finished generating $FONT_NAME.aff ---"
echo "Please Manually Create the <Space> Character in .aff File"
