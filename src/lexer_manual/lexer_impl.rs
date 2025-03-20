use crate::lexer_manual::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    /// Cria um novo Lexer a partir de uma string de entrada.
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    /// Retorna o próximo token encontrado na entrada.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        let current = self.input[self.pos];

        // Tratamento de comentários (// comentário)
        if current == '/' && self.peek() == Some('/') {
            self.pos += 2; // pula os dois caracteres '/'
            let start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                self.pos += 1;
            }
            let comment: String = self.input[start..self.pos].iter().collect();
            return Token::Comment(comment);
        }

        // Números inteiros
        if current.is_ascii_digit() {
            let start = self.pos;
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
            let num_str: String = self.input[start..self.pos].iter().collect();
            if let Ok(num) = num_str.parse::<i64>() {
                return Token::Number(num);
            }
        }

        // Identificadores e palavras-chave
        if current.is_ascii_alphabetic() || current == '_' {
            let start = self.pos;
            while self.pos < self.input.len()
                && (self.input[self.pos].is_ascii_alphanumeric() || self.input[self.pos] == '_')
            {
                self.pos += 1;
            }
            let word: String = self.input[start..self.pos].iter().collect();
            return match word.as_str() {
                "move_up" => Token::MoveUp,
                "move_down" => Token::MoveDown,
                "move_left" => Token::MoveLeft,
                "move_right" => Token::MoveRight,
                "jump" => Token::Jump,
                "attack" => Token::Attack,
                "defend" => Token::Defend,
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,
                "for" => Token::For,
                _ => Token::Identifier(word),
            };
        }

        // Operadores e símbolos especiais
        match current {
            '+' => {
                self.pos += 1;
                Token::Plus
            }
            '-' => {
                self.pos += 1;
                Token::Minus
            }
            '*' => {
                self.pos += 1;
                Token::Asterisk
            }
            '/' => {
                self.pos += 1;
                Token::Slash
            }
            '(' => {
                self.pos += 1;
                Token::LParen
            }
            ')' => {
                self.pos += 1;
                Token::RParen
            }
            '{' => {
                self.pos += 1;
                Token::LBrace
            }
            '}' => {
                self.pos += 1;
                Token::RBrace
            }
            '!' => {
                self.pos += 1;
                Token::LogicalNot
            }
            '&' => {
                if self.peek() == Some('&') {
                    self.pos += 2;
                    Token::LogicalAnd
                } else {
                    self.pos += 1;
                    // Caso simples: se & aparecer sozinho, tratar como LogicalAnd
                    Token::LogicalAnd
                }
            }
            '|' => {
                if self.peek() == Some('|') {
                    self.pos += 2;
                    Token::LogicalOr
                } else {
                    self.pos += 1;
                    // Similarmente, caso de uso isolado de '|'
                    Token::LogicalOr
                }
            }
            _ => {
                // Se o caractere não for reconhecido, pula-o e retorna um identificador com o char
                self.pos += 1;
                Token::Identifier(current.to_string())
            }
        }
    }

    /// Pula espaços em branco e outros separadores
    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    /// Olha o próximo caractere sem incrementar a posição.
    fn peek(&self) -> Option<char> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }
}

pub fn lex(input: &str) -> Vec<Token> {
    // Exemplo simples: para cada palavra, mapeia para um token se possível.
    input.split_whitespace().map(|s| {
        match s {
            "if"    => Token::If,
            "else"  => Token::Else,
            "while" => Token::While,
            "for"   => Token::For,
            "move_up"    => Token::MoveUp,
            "move_down"  => Token::MoveDown,
            "move_left"  => Token::MoveLeft,
            "move_right" => Token::MoveRight,
            "jump"       => Token::Jump,
            "attack"     => Token::Attack,
            "defend"     => Token::Defend,
            // Se for um número, por exemplo
            _ if s.chars().all(|c| c.is_digit(10)) => {
                // Aqui, assumindo que o parse não falha
                Token::Number(s.parse::<i64>().unwrap())
            }
            // Para identificadores (hero, enemy, etc.) ou outros casos
            _ => Token::Identifier(s.to_string()),
        }
    }).collect()
}