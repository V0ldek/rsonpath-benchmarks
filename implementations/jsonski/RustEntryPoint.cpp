#include "RustEntryPoint.h"
#include "RecordLoader.h"
#include "QueryProcessor.h"
#include <iostream>

Record *loadFile(char *file_path)
{
    Record *rec = RecordLoader::loadSingleRecord(file_path);

    if (rec == NULL)
    {
        std::cerr << "panic: failed to load record\n";
        exit(1);
    }

    return rec;
}

Result *runJsonSki(char *query, Record *record)
{
    QueryProcessor processor(query);

    return new Result(processor.runQuery(record));
}

void dropFile(Record *record)
{
    delete record;
}

void dropResult(Result *result)
{
    delete result;
}
