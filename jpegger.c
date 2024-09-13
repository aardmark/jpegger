#include <stdio.h>
#include <stdlib.h>
#include <dirent.h>

int isValidJpeg(char* fileName);

int main(int argc, char const *argv[])
{
    size_t fileSize;
    DIR *dp;
    struct dirent *ep;

    dp = opendir("./");
    if (dp != NULL) {
        while ((ep = readdir(dp)) != NULL)
        {
            if (ep->d_name[0] != '.') {
                if (isValidJpeg(ep->d_name)) {
                    printf("%s is a valid jpeg.\n", ep->d_name);
                } else {
                    printf("%s is not a valid jpeg.\n", ep->d_name);
                }
            }
        }
        closedir(dp);
    } else {
        printf("Unable to open folder.");
    }

    return 0;
}

int isValidJpeg(char* fileName) {
    size_t fileSize;
    FILE *fp;

    fp = fopen(fileName, "rb");
    if (fp == NULL) return 0;

    fseek(fp, 0L, SEEK_END);
    fileSize = ftell(fp);
    if (fileSize < 4)
    {
        fclose(fp);
        return 0;    
    }

    fseek(fp, 0L, SEEK_SET);
    int firstByte = fgetc(fp);
    int secondByte = fgetc(fp);
    if (!(firstByte == 0xFF && secondByte == 0xD8)) {
        fclose(fp);
        return 0;    
    }

    fseek(fp, -2, SEEK_END);
    firstByte = fgetc(fp);
    secondByte = fgetc(fp);
    if (!(firstByte == 0xFF && secondByte == 0xD9)) {
        fclose(fp);
        return 0;    
    }

    fclose(fp);
    return 1;
}
