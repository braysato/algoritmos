#include <iostream>
#include <string>
#include <iomanip>

using namespace std;

string months[] = {"", "JAN", "FEV", "MAR", "ABR", "MAI", "JUN", "JUL", "AGO", "SET", "OUT", "NOV", "DEZ"};
string days[] = {"DOM", "SEG", "TER", "QUA", "QUI", "SEX", "SAB"};

int getMonthNumber(string month) {
    for (int i = 1; i <= 12; i++) {
        if (months[i] == month) return i;
    }
    return 1;
}

bool isLeapYear(int year) {
    return (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
}

int getDaysInMonth(int month, int year) {
    int daysInMonth[] = {0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    if (month == 2 && isLeapYear(year)) return 29;
    return daysInMonth[month];
}

int getDayOfWeek(int d, int m, int y) {
    int t[] = {0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4};
    y -= m < 3;
    return (y + y/4 - y/100 + y/400 + t[m-1] + d) % 7;
}

void addSeconds(int &day, int &month, int &year, int &hour, int &minute, int &second, int addSec) {
    second += addSec;
    
    minute += second / 60;
    second %= 60;
    
    hour += minute / 60;
    minute %= 60;
    
    day += hour / 24;
    hour %= 24;
    
    while (day > getDaysInMonth(month, year)) {
        day -= getDaysInMonth(month, year);
        month++;
        if (month > 12) {
            month = 1;
            year++;
        }
    }
}

int main() {
    string line;
    int caseNumber = 1;
    
    while (getline(cin, line)) {
        if (line == "FIM") break;
        
        string dayOfWeek = line.substr(0, 3);
        int day = stoi(line.substr(4, 2));
        string month = line.substr(6, 3);
        int year = stoi(line.substr(9, 4));
        int hour = stoi(line.substr(14, 2));
        int minute = stoi(line.substr(17, 2));
        int second = stoi(line.substr(20, 2));
        
        int monthNum = getMonthNumber(month);
        
        cout << "Previsao #" << caseNumber << endl;
        
        for (int i = 0; i < 5; i++) {
            int seconds;
            cin >> seconds;
            cin.ignore();
            
            int newDay = day;
            int newMonth = monthNum;
            int newYear = year;
            int newHour = hour;
            int newMinute = minute;
            int newSecond = second;
            
            addSeconds(newDay, newMonth, newYear, newHour, newMinute, newSecond, seconds);
            
            int dayOfWeekNum = getDayOfWeek(newDay, newMonth, newYear);
            
            cout << days[dayOfWeekNum] << ","
                 << setfill('0') << setw(2) << newDay
                 << months[newMonth]
                 << newYear << ":"
                 << setfill('0') << setw(2) << newHour << ":"
                 << setfill('0') << setw(2) << newMinute << ":"
                 << setfill('0') << setw(2) << newSecond << endl;
        }
        
        caseNumber++;
    }
    
    return 0;
}
