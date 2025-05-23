#!/bin/bash

# Prints my project tree, replaces existing tree

OUTPUT_DIR="Architecture"
OUTPUT_FILE="${OUTPUT_DIR}/TREE.md"


GREEN='\033[0;32m'
BRIGHT_GREEN='\033[1;32m' 
BROWN='\033[0;33m'         
CYAN='\033[0;36m'
NC='\033[0m'              


echo -e "${CYAN}Current directory: $(pwd)${NC}"
echo -e "${CYAN}Generating project tree, excluding .gitignore and CLI...${NC}"
echo -e "${CYAN}Output will be saved to ./${OUTPUT_FILE} (existing content will be overwritten)${NC}"


tree -I ".gitignore|CLI" > "${OUTPUT_FILE}"


if [ -s "${OUTPUT_FILE}" ]; then
    echo "" 

    echo -e "${GREEN}         *         ${NC}"
    echo -e "${GREEN}        / \\        ${NC}"
    echo -e "${GREEN}       / _ \\       ${NC}"
    echo -e "${GREEN}      / ___ \\      ${NC}"
    echo -e "${GREEN}     /_______\\     ${NC}"
    echo -e "${BROWN}        |||        ${NC}"
    echo -e "${BROWN}       -----       ${NC}"
    echo -e "${BRIGHT_GREEN}--------------------------------------------------${NC}"
    echo -e "${BRIGHT_GREEN} Success! Project tree updated! ${NC}"
    echo -e "${BRIGHT_GREEN}--------------------------------------------------${NC}"
    echo -e "${CYAN}Tree structure saved to:${NC} $(pwd)/${OUTPUT_FILE}"
    echo "" 

elif [ -f "${OUTPUT_FILE}" ]; then
    echo ""
    echo -e "\033[0;33mWarning: Tree command ran, but output file ./${OUTPUT_FILE} is empty.\033[0m"
    echo -e "\033[0;33mThis might happen if 'tree' found nothing to list after exclusions, or an error occurred.\033[0m"
else
    echo ""
    echo -e "\033[0;31mError: Failed to generate project tree or save to ./${OUTPUT_FILE}.\033[0m"
    echo -e "\033[0;31mMake sure 'tree' command is installed and you have write permissions here.\033[0m"
    exit 1
fi

exit 0
