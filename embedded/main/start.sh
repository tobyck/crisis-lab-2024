if [[ -z $1 ]] then
	echo "Usage: ./start.sh [STAGES] [PORT]

Stages are \`c\` to compile, \`l\` to load, and \`m\` to monitor serial output. 
You must specify at least one of these, but you can specify more, and in any order.

Examples:
  ./start.sh c                      Only compile
  ./start.sh lm /dev/ttyUSB0        Load code onto /dev/ttyUSB0 and monitor output
  ./start.sh cl /dev/ttyUSB0        Compile and load onto the board on /dev/ttyUSB0
  ./start.sh m COM4                 Monitor board on COM4
  ./start.sh clm COM3               Compile, load, and monitor on COM3"
fi

compile="arduino-cli compile --fqbn SparkFun:avr:RedBoard ."
load="arduino-cli upload -p $2 --fqbn arduino:avr:uno ."
monitor="arduino-cli monitor -p $2 --config baudrate=115200"

if [[ $1 == *"c"* ]]; then eval "$compile"; fi
if [[ $1 == *"l"* ]]; then eval "$load"; fi
if [[ $1 == *"m"* ]]; then eval "$monitor"; fi
