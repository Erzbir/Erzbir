#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct AboutMe {
    char *name;
    char **languages;
    char **hobbies;
    char **learnings;
    char *blog;
    char *email;
} AboutMe;

AboutMe *user = NULL;
char *name = "erzbir";
char *blog = "erzbir.com";
char *email = "erzbir@mail.com";
char *languages[] = {"Java", "Python", "JavaScript", "Html", "C#", "C", NULL};
char *hobbies[] = {"Irish flute", "Tin whistle", "Yu-Gi-Oh OCG", "Riding", NULL};
char *learnings[] = {"JVM", "Spring Cloud", "Kotlin", "Vue", NULL};

void init();
void print_arr(char **s);
void learn(char *s);
void clean();
int len_arr(char **s);

int main() {
    init();
    printf("About Me:\n");
    printf("  name: \n    %s\n", user->name);
    printf("  languages:\n    ");
    print_arr(user->languages);
    printf("  hobbies:\n    ");
    print_arr(user->hobbies);
    learn("Assembly");
    learn("everything");
    learn("......");
    printf("  learnings:\n    ");
    print_arr(user->learnings);
    printf("  blog:\n    %s\n", user->blog);
    printf("  email:\n    %s", user->email);
    clean();
    return 0;
}

void init() {
    user = malloc(sizeof(AboutMe));
    user->name = name;
    user->blog = blog;
    user->email = email;
    user->languages = malloc(sizeof(languages));
    user->hobbies = malloc(sizeof(hobbies));
    user->learnings = malloc(sizeof(learnings));
    memcpy(user->languages, languages, sizeof(languages));
    memcpy(user->hobbies, hobbies, sizeof(hobbies));
    memcpy(user->learnings, learnings, sizeof(learnings));
}

void learn(char *s) {
    if (s == NULL) return;
    char **ptr = user->learnings;
    int length = len_arr(user->learnings);
    user->learnings = realloc(ptr, sizeof(char *) * (length + 1));
    ptr = user->learnings;
    ptr[length] = s;
}

int len_arr(char **s) {
    if (s == NULL) return 0;
    int length = 0;
    char **ptr = s;
    while (*ptr++ != NULL) length++;
    return length;
}

void print_arr(char **s) {
    if (s == NULL) return;
    char **ptr = s;
    printf("[");
    while (1) {
        printf("'%s'", *ptr);
        ptr++;
        if (*ptr != NULL) printf(", ");
        else break;
    }
    printf("]\n");
}

void clean() {
    free(user->languages);
    free(user->hobbies);
    free(user->learnings);
    free(user);
}