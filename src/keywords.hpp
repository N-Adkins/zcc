#pragma once

#include <string>
#include <unordered_map>

namespace ZCC 
{
    
    enum class Keyword 
    {
        Auto,
        Break,
        Case,
        Char,
        Const,
        Continue,
        Default,
        Do,
        Double,
        Else,
        Enum,
        Extern,
        Float,
        For,
        Goto,
        If,
        Int,
        Long,
        Register,
        Return,
        Short,
        Signed,
        Sizeof,
        Static,
        Struct,
        Switch,
        Typedef,
        Union,
        Unsigned,
        Void,
        Volatile,
        While,
    };

    static const std::unordered_map<std::string, Keyword> string_to_keyword = {{
        {"auto", Keyword::Auto},
        {"break", Keyword::Break},
        {"case", Keyword::Case},
        {"char", Keyword::Char},
        {"const", Keyword::Const},
        {"continue", Keyword::Continue},
        {"default", Keyword::Default},
        {"do", Keyword::Do},
        {"double", Keyword::Double},
        {"else", Keyword::Else},
        {"enum", Keyword::Enum},
        {"extern", Keyword::Extern},
        {"float", Keyword::Float},
        {"for", Keyword::For},
        {"goto", Keyword::Goto},
        {"if", Keyword::If},
        {"int", Keyword::Int},
        {"long", Keyword::Long},
        {"register", Keyword::Register},
        {"return", Keyword::Return},
        {"short", Keyword::Short},
        {"signed", Keyword::Signed},
        {"sizeof", Keyword::Sizeof},
        {"static", Keyword::Static},
        {"struct", Keyword::Struct},
        {"switch", Keyword::Switch},
        {"typedef", Keyword::Typedef},
        {"union", Keyword::Union},
        {"unsigned", Keyword::Unsigned},
        {"void", Keyword::Void},
        {"volatile", Keyword::Volatile},
        {"while", Keyword::While},
    }};

}
