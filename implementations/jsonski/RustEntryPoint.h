#include "Records.h"

class Result {
private:
    std::string str;

public:
    Result(std::string x) : str(x) {}
};

extern "C" Record *loadFile(char *file_path);
extern "C" Result *runJsonSki(char *query, Record *record);
extern "C" void dropFile(Record *record);
extern "C" void dropResult(Result *result);