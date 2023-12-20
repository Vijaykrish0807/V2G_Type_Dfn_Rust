
/* SPDX-License-Identifier: Apache-2.0 */
/*
 * Copyright (C) 2022 - 2023 chargebyte GmbH
 * Copyright (C) 2022 - 2023 Contributors to EVerest
 */

/*****************************************************
 *
 * @author
 * @version
 *
 * The Code is generated! Changes may be overwritten.
 *
 *****************************************************/

/**
  * @file iso2_msgDefEncoder.c
  * @brief Description goes here
  *
  **/
use crate::*;
use exigen_core::{*,error::*};
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Cost; type={urn:iso:15118:2:2013:MsgDataTypes}CostType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: costKind, costKindType (1, 1); amount, unsignedInt (1, 1); amountMultiplier, unitMultiplierType (0, 1);
  pub fn encode_ISO2CostType(stream: &mut ExiBitstream, CostType: &ISO2CostType )->Result<(), BitstreamError> {
      let mut grammar_id = 0;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           0=>{
               // Grammar: ID=0; read/write bits=1; START (costKind)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=1
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, CostType.costKind as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 1;
  }
           1=>{
               // Grammar: ID=1; read/write bits=1; START (amount)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedLong); next=2
  
  
                  stream.write_nbit_uint( 1, 0)?;
                  stream.write_u32(CostType.amount as u32)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 2;
  
  
  }
           2=>{
               // Grammar: ID=2; read/write bits=2; START (amountMultiplier), END Element
              if CostType.amountMultiplier.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (amountMultiplier, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?; 
  
                      stream.write_nbit_uint(3, CostType.amountMultiplier.unwrap() as u32 + 3)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Transform; type={http://www.w3.org/2000/09/xmldsig#}TransformType; base type=; content type=mixed;
  //          abstract=False; final=False; choice=True;
  // Particle: Algorithm, anyURI (1, 1); ANY, anyType (0, 1); XPath, string (0, 1);
  pub fn encode_ISO2TransformType(stream: &mut ExiBitstream, TransformType: &ISO2TransformType )->Result<(), BitstreamError> {
      let mut grammar_id = 5;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           5=>{
               // Grammar: ID=5; read/write bits=1; START (Algorithm)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (anyURI); next=6
  
              // string should not be found in table, so add 2
  
               stream.write_u16(TransformType.Algorithm.len() as u16)?;
              stream.write_characters(&TransformType.Algorithm.to_string(), ISO2Algorithm_CHARACTER_SIZE)?;
  
              grammar_id = 6;
  }
           6=>{
               // Grammar: ID=6; read/write bits=3; START (XPath), START (ANY), END Element, START (ANY)
              if TransformType.XPath.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (XPath, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(TransformType.XPath.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&TransformType.XPath.expect("Value not Initialized").to_string(), ISO2XPath_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=1)
              //{
              // ***** //
              else if TransformType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(TransformType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &TransformType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}TimeInterval; type={urn:iso:15118:2:2013:MsgDataTypes}IntervalType; base type=; content type=empty;
  //          abstract=True; final=False;
  fn encode_ISO2IntervalType(stream: &mut ExiBitstream,IntervalType: &ISO2IntervalType)->Result<(),BitstreamError>{
      // Element has no particles, so the function just encodes END Element
      let _ = IntervalType;// To silence the unused variable warning
  
  //    error:i32= exi_basetypes_encoder_nbit_uint(stream, 1, 0);
      stream.write_nbit_uint(1,0)?;
       Ok(())
  }
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Transforms; type={http://www.w3.org/2000/09/xmldsig#}TransformsType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Transform, TransformType (1, 1);
  pub fn encode_ISO2TransformsType(stream: &mut ExiBitstream, TransformsType: &ISO2TransformsType )->Result<(), BitstreamError> {
      let mut grammar_id = 7;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           7=>{
               // Grammar: ID=7; read/write bits=1; START (Transform)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (TransformType); next=8
  
  
  
                  encode_ISO2TransformType(stream,&TransformsType.Transform)?;
  
                  grammar_id = 8;
  }
           8=>{
               // Grammar: ID=8; read/write bits=2; START (Transform), END Element
              if (1 == 0)
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Transform, TransformType); next=3
  
  
  
                      encode_ISO2TransformType(stream,&TransformsType.Transform)?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}DSAKeyValue; type={http://www.w3.org/2000/09/xmldsig#}DSAKeyValueType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: P, CryptoBinary (0, 1)(was 1, 1)(seq. ['P', 'Q']); Q, CryptoBinary (0, 1)(was 1, 1)(seq. ['P', 'Q']); G, CryptoBinary (0, 1); Y, CryptoBinary (1, 1); J, CryptoBinary (0, 1); Seed, CryptoBinary (0, 1)(was 1, 1)(seq. ['Seed', 'PgenCounter']); PgenCounter, CryptoBinary (0, 1)(was 1, 1)(seq. ['Seed', 'PgenCounter']);
  pub fn encode_ISO2DSAKeyValueType(stream: &mut ExiBitstream, DSAKeyValueType: &ISO2DSAKeyValueType )->Result<(), BitstreamError> {
      let mut grammar_id = 9;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           9=>{
               // Grammar: ID=9; read/write bits=2; START (P), START (G), START (Y)
              if DSAKeyValueType.P.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (P, base64Binary); next=10
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.P.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.P.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 10;
  
  
  
              }
              else if DSAKeyValueType.G.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (G, base64Binary); next=12
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.G.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.G.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 12;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (Y, base64Binary); next=13
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.Y.len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.Y)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 13;
  
  
  
              }
  }
           10=>{
               // Grammar: ID=10; read/write bits=1; START (Q)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=11
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(DSAKeyValueType.Q.expect("Value not Initialized").len() as u16)?;
                          stream.write_bytes( &DSAKeyValueType.Q.expect("Value not Initialized"))?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 11;
  
  
  }
           11=>{
               // Grammar: ID=11; read/write bits=2; START (G), START (Y)
              if DSAKeyValueType.G.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (G, base64Binary); next=12
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.G.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.G.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 12;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (Y, base64Binary); next=13
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.Y.len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.Y)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 13;
  
  
  
              }
  }
           12=>{
               // Grammar: ID=12; read/write bits=1; START (Y)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=13
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(DSAKeyValueType.Y.len() as u16)?;
                          stream.write_bytes( &DSAKeyValueType.Y)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 13;
  
  
  }
           13=>{
               // Grammar: ID=13; read/write bits=2; START (J), START (Seed), END Element
              if DSAKeyValueType.J.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (J, base64Binary); next=14
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.J.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.J.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 14;
  
  
  
              }
              else if DSAKeyValueType.Seed.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (Seed, base64Binary); next=15
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.Seed.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.Seed.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 15;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           14=>{
               // Grammar: ID=14; read/write bits=2; START (Seed), END Element
              if DSAKeyValueType.Seed.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Seed, base64Binary); next=15
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.Seed.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.Seed.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 15;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           15=>{
               // Grammar: ID=15; read/write bits=2; START (PgenCounter), END Element
              if DSAKeyValueType.PgenCounter.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (PgenCounter, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.PgenCounter.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.PgenCounter.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}X509IssuerSerial; type={http://www.w3.org/2000/09/xmldsig#}X509IssuerSerialType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: X509IssuerName, string (1, 1); X509SerialNumber, integer (1, 1);
  pub fn encode_ISO2X509IssuerSerialType(stream: &mut ExiBitstream, X509IssuerSerialType: &ISO2X509IssuerSerialType )->Result<(), BitstreamError> {
      let mut grammar_id = 16;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           16=>{
               // Grammar: ID=16; read/write bits=1; START (X509IssuerName)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=17
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
  
                   stream.write_u16(X509IssuerSerialType.X509IssuerName.len() as u16)?;
                  stream.write_characters(&X509IssuerSerialType.X509IssuerName.to_string(), ISO2X509IssuerName_CHARACTER_SIZE)?;
  
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 17;
  }
           17=>{
               // Grammar: ID=17; read/write bits=1; START (X509SerialNumber)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (decimal); next=3
  
  
                  stream.write_nbit_uint( 1, 0)?;
                  stream.write_i32(X509IssuerSerialType.X509SerialNumber as write_i32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}RelativeTimeInterval; type={urn:iso:15118:2:2013:MsgDataTypes}RelativeTimeIntervalType; base type=IntervalType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: start, AnonType (1, 1); duration, AnonType (0, 1);
  pub fn encode_ISO2RelativeTimeIntervalType(stream: &mut ExiBitstream, RelativeTimeIntervalType: &ISO2RelativeTimeIntervalType )->Result<(), BitstreamError> {
      let mut grammar_id = 18;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           18=>{
               // Grammar: ID=18; read/write bits=1; START (start)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedInt); next=19
  
  
                  stream.write_nbit_uint( 1, 0)?;
                  stream.write_u32(RelativeTimeIntervalType.start as u32)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 19;
  
  
  }
           19=>{
               // Grammar: ID=19; read/write bits=2; START (duration), END Element
              if RelativeTimeIntervalType.duration.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (duration, unsignedInt); next=3
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_u32(RelativeTimeIntervalType.duration as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}DigestMethod; type={http://www.w3.org/2000/09/xmldsig#}DigestMethodType; base type=; content type=mixed;
  //          abstract=False; final=False;
  // Particle: Algorithm, anyURI (1, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2DigestMethodType(stream: &mut ExiBitstream, DigestMethodType: &ISO2DigestMethodType )->Result<(), BitstreamError> {
      let mut grammar_id = 20;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           20=>{
               // Grammar: ID=20; read/write bits=1; START (Algorithm)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (anyURI); next=21
  
              // string should not be found in table, so add 2
  
               stream.write_u16(DigestMethodType.Algorithm.len() as u16)?;
              stream.write_characters(&DigestMethodType.Algorithm.to_string(), ISO2Algorithm_CHARACTER_SIZE)?;
  
              grammar_id = 21;
  }
           21=>{
               // Grammar: ID=21; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if DigestMethodType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DigestMethodType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &DigestMethodType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}RSAKeyValue; type={http://www.w3.org/2000/09/xmldsig#}RSAKeyValueType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Modulus, CryptoBinary (1, 1); Exponent, CryptoBinary (1, 1);
  pub fn encode_ISO2RSAKeyValueType(stream: &mut ExiBitstream, RSAKeyValueType: &ISO2RSAKeyValueType )->Result<(), BitstreamError> {
      let mut grammar_id = 22;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           22=>{
               // Grammar: ID=22; read/write bits=1; START (Modulus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=23
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(RSAKeyValueType.Modulus.len() as u16)?;
                          stream.write_bytes( &RSAKeyValueType.Modulus)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 23;
  
  
  }
           23=>{
               // Grammar: ID=23; read/write bits=1; START (Exponent)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(RSAKeyValueType.Exponent.len() as u16)?;
                          stream.write_bytes( &RSAKeyValueType.Exponent)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}CanonicalizationMethod; type={http://www.w3.org/2000/09/xmldsig#}CanonicalizationMethodType; base type=; content type=mixed;
  //          abstract=False; final=False;
  // Particle: Algorithm, anyURI (1, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2CanonicalizationMethodType(stream: &mut ExiBitstream, CanonicalizationMethodType: &ISO2CanonicalizationMethodType )->Result<(), BitstreamError> {
      let mut grammar_id = 24;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           24=>{
               // Grammar: ID=24; read/write bits=1; START (Algorithm)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (anyURI); next=25
  
              // string should not be found in table, so add 2
  
               stream.write_u16(CanonicalizationMethodType.Algorithm.len() as u16)?;
              stream.write_characters(&CanonicalizationMethodType.Algorithm.to_string(), ISO2Algorithm_CHARACTER_SIZE)?;
  
              grammar_id = 25;
  }
           25=>{
               // Grammar: ID=25; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if CanonicalizationMethodType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(CanonicalizationMethodType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &CanonicalizationMethodType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignatureMethod; type={http://www.w3.org/2000/09/xmldsig#}SignatureMethodType; base type=; content type=mixed;
  //          abstract=False; final=False;
  // Particle: Algorithm, anyURI (1, 1); HMACOutputLength, HMACOutputLengthType (0, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2SignatureMethodType(stream: &mut ExiBitstream, SignatureMethodType: &ISO2SignatureMethodType )->Result<(), BitstreamError> {
      let mut grammar_id = 26;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           26=>{
               // Grammar: ID=26; read/write bits=1; START (Algorithm)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (anyURI); next=27
  
              // string should not be found in table, so add 2
  
               stream.write_u16(SignatureMethodType.Algorithm.len() as u16)?;
              stream.write_characters(&SignatureMethodType.Algorithm.to_string(), ISO2Algorithm_CHARACTER_SIZE)?;
  
              grammar_id = 27;
  }
           27=>{
               // Grammar: ID=27; read/write bits=3; START (HMACOutputLength), START (ANY), END Element, START (ANY)
              if SignatureMethodType.HMACOutputLength.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (HMACOutputLength, integer); next=28
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_i32(SignatureMethodType.HMACOutputLength as write_i32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 28;
  
  
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=1)
              //{
              // ***** //
              else if SignatureMethodType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SignatureMethodType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &SignatureMethodType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           28=>{
               // Grammar: ID=28; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if SignatureMethodType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SignatureMethodType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &SignatureMethodType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}KeyValue; type={http://www.w3.org/2000/09/xmldsig#}KeyValueType; base type=; content type=mixed;
  //          abstract=False; final=False; choice=True;
  // Particle: DSAKeyValue, DSAKeyValueType (0, 1); RSAKeyValue, RSAKeyValueType (0, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2KeyValueType(stream: &mut ExiBitstream, KeyValueType: &ISO2KeyValueType )->Result<(), BitstreamError> {
      let mut grammar_id = 29;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           29=>{
               // Grammar: ID=29; read/write bits=2; START (DSAKeyValue), START (RSAKeyValue), START (ANY)
              if KeyValueType.DSAKeyValue.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (DSAKeyValue, DSAKeyValueType); next=3
  
  
   
                      encode_ISO2DSAKeyValueType(stream,&KeyValueType.DSAKeyValue.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyValueType.RSAKeyValue.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (RSAKeyValue, RSAKeyValueType); next=3
  
  
   
                      encode_ISO2RSAKeyValueType(stream,&KeyValueType.RSAKeyValue.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyValueType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(KeyValueType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &KeyValueType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Reference; type={http://www.w3.org/2000/09/xmldsig#}ReferenceType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); Type, anyURI (0, 1); URI, anyURI (0, 1); Transforms, TransformsType (0, 1); DigestMethod, DigestMethodType (1, 1); DigestValue, DigestValueType (1, 1);
  pub fn encode_ISO2ReferenceType(stream: &mut ExiBitstream, ReferenceType: &ISO2ReferenceType )->Result<(), BitstreamError> {
      let mut grammar_id = 30;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           30=>{
               // Grammar: ID=30; read/write bits=3; START (Id), START (Type), START (URI), START (Transforms), START (DigestMethod)
              if ReferenceType.Id.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (Id, NCName); next=31
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ReferenceType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ReferenceType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 31;
  
              }
              else if ReferenceType.Type.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (Type, anyURI); next=32
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ReferenceType.Type.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ReferenceType.Type.expect("Value not Initialized").to_string(), ISO2Type_CHARACTER_SIZE)?;
  
                  grammar_id = 32;
  
              }
              else if ReferenceType.URI.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (URI, anyURI); next=33
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ReferenceType.URI.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ReferenceType.URI.expect("Value not Initialized").to_string(), ISO2URI_CHARACTER_SIZE)?;
  
                  grammar_id = 33;
  
              }
              else if ReferenceType.Transforms.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (Transforms, TransformsType); next=34
  
  
   
                      encode_ISO2TransformsType(stream,&ReferenceType.Transforms.unwrap())?;
  
                      grammar_id = 34;
              }
              else
              {
                  stream.write_nbit_uint(3, 4 as u32)?;
                  // Event: START (DigestMethod, DigestMethodType); next=35
  
  
  
                      encode_ISO2DigestMethodType(stream,&ReferenceType.DigestMethod)?;
  
                      grammar_id = 35;
              }
  }
           31=>{
               // Grammar: ID=31; read/write bits=3; START (Type), START (URI), START (Transforms), START (DigestMethod)
              if ReferenceType.Type.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (Type, anyURI); next=32
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ReferenceType.Type.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ReferenceType.Type.expect("Value not Initialized").to_string(), ISO2Type_CHARACTER_SIZE)?;
  
                  grammar_id = 32;
  
              }
              else if ReferenceType.URI.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (URI, anyURI); next=33
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ReferenceType.URI.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ReferenceType.URI.expect("Value not Initialized").to_string(), ISO2URI_CHARACTER_SIZE)?;
  
                  grammar_id = 33;
  
              }
              else if ReferenceType.Transforms.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (Transforms, TransformsType); next=34
  
  
   
                      encode_ISO2TransformsType(stream,&ReferenceType.Transforms.unwrap())?;
  
                      grammar_id = 34;
              }
              else
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (DigestMethod, DigestMethodType); next=35
  
  
  
                      encode_ISO2DigestMethodType(stream,&ReferenceType.DigestMethod)?;
  
                      grammar_id = 35;
              }
  }
           32=>{
               // Grammar: ID=32; read/write bits=2; START (URI), START (Transforms), START (DigestMethod)
              if ReferenceType.URI.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (URI, anyURI); next=33
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ReferenceType.URI.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ReferenceType.URI.expect("Value not Initialized").to_string(), ISO2URI_CHARACTER_SIZE)?;
  
                  grammar_id = 33;
  
              }
              else if ReferenceType.Transforms.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (Transforms, TransformsType); next=34
  
  
   
                      encode_ISO2TransformsType(stream,&ReferenceType.Transforms.unwrap())?;
  
                      grammar_id = 34;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (DigestMethod, DigestMethodType); next=35
  
  
  
                      encode_ISO2DigestMethodType(stream,&ReferenceType.DigestMethod)?;
  
                      grammar_id = 35;
              }
  }
           33=>{
               // Grammar: ID=33; read/write bits=2; START (Transforms), START (DigestMethod)
              if ReferenceType.Transforms.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Transforms, TransformsType); next=34
  
  
   
                      encode_ISO2TransformsType(stream,&ReferenceType.Transforms.unwrap())?;
  
                      grammar_id = 34;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (DigestMethod, DigestMethodType); next=35
  
  
  
                      encode_ISO2DigestMethodType(stream,&ReferenceType.DigestMethod)?;
  
                      grammar_id = 35;
              }
  }
           34=>{
               // Grammar: ID=34; read/write bits=1; START (DigestMethod)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (DigestMethodType); next=35
  
  
  
                  encode_ISO2DigestMethodType(stream,&ReferenceType.DigestMethod)?;
  
                  grammar_id = 35;
  }
           35=>{
               // Grammar: ID=35; read/write bits=1; START (DigestValue)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(ReferenceType.DigestValue.len() as u16)?;
                          stream.write_bytes( &ReferenceType.DigestValue)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}RetrievalMethod; type={http://www.w3.org/2000/09/xmldsig#}RetrievalMethodType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Type, anyURI (0, 1); URI, anyURI (0, 1); Transforms, TransformsType (0, 1);
  pub fn encode_ISO2RetrievalMethodType(stream: &mut ExiBitstream, RetrievalMethodType: &ISO2RetrievalMethodType )->Result<(), BitstreamError> {
      let mut grammar_id = 36;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           36=>{
               // Grammar: ID=36; read/write bits=3; START (Type), START (URI), START (Transforms), END Element
              if RetrievalMethodType.Type.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (Type, anyURI); next=37
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(RetrievalMethodType.Type.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&RetrievalMethodType.Type.expect("Value not Initialized").to_string(), ISO2Type_CHARACTER_SIZE)?;
  
                  grammar_id = 37;
  
              }
              else if RetrievalMethodType.URI.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (URI, anyURI); next=38
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(RetrievalMethodType.URI.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&RetrievalMethodType.URI.expect("Value not Initialized").to_string(), ISO2URI_CHARACTER_SIZE)?;
  
                  grammar_id = 38;
  
              }
              else if RetrievalMethodType.Transforms.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (Transforms, TransformsType); next=3
  
  
   
                      encode_ISO2TransformsType(stream,&RetrievalMethodType.Transforms.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           37=>{
               // Grammar: ID=37; read/write bits=2; START (URI), START (Transforms), END Element
              if RetrievalMethodType.URI.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (URI, anyURI); next=38
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(RetrievalMethodType.URI.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&RetrievalMethodType.URI.expect("Value not Initialized").to_string(), ISO2URI_CHARACTER_SIZE)?;
  
                  grammar_id = 38;
  
              }
              else if RetrievalMethodType.Transforms.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (Transforms, TransformsType); next=3
  
  
   
                      encode_ISO2TransformsType(stream,&RetrievalMethodType.Transforms.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           38=>{
               // Grammar: ID=38; read/write bits=2; START (Transforms), END Element
              if RetrievalMethodType.Transforms.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Transforms, TransformsType); next=3
  
  
   
                      encode_ISO2TransformsType(stream,&RetrievalMethodType.Transforms.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}X509Data; type={http://www.w3.org/2000/09/xmldsig#}X509DataType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: X509IssuerSerial, X509IssuerSerialType (0, 1); X509SKI, base64Binary (0, 1); X509SubjectName, string (0, 1); X509Certificate, base64Binary (0, 1); X509CRL, base64Binary (0, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2X509DataType(stream: &mut ExiBitstream, X509DataType: &ISO2X509DataType )->Result<(), BitstreamError> {
      let mut grammar_id = 39;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           39=>{
               // Grammar: ID=39; read/write bits=3; START (X509IssuerSerial), START (X509SKI), START (X509SubjectName), START (X509Certificate), START (X509CRL), START (ANY)
              if X509DataType.X509IssuerSerial.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (X509IssuerSerial, X509IssuerSerialType); next=3
  
  
   
                      encode_ISO2X509IssuerSerialType(stream,&X509DataType.X509IssuerSerial.unwrap())?;
  
                      grammar_id = 3;
              }
              else if X509DataType.X509SKI.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (X509SKI, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(X509DataType.X509SKI.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &X509DataType.X509SKI.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else if X509DataType.X509SubjectName.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (X509SubjectName, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(X509DataType.X509SubjectName.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&X509DataType.X509SubjectName.expect("Value not Initialized").to_string(), ISO2X509SubjectName_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else if X509DataType.X509Certificate.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (X509Certificate, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(X509DataType.X509Certificate.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &X509DataType.X509Certificate.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else if X509DataType.X509CRL.is_some()
              {
                  stream.write_nbit_uint(3, 4 as u32)?;
                  // Event: START (X509CRL, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(X509DataType.X509CRL.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &X509DataType.X509CRL.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else if X509DataType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 5 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(X509DataType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &X509DataType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}PGPData; type={http://www.w3.org/2000/09/xmldsig#}PGPDataType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; choice=True; sequence=True (2;
  // Particle: PGPKeyID, base64Binary (1, 1); PGPKeyPacket, base64Binary (0, 1); ANY, anyType (0, 1); PGPKeyPacket, base64Binary (1, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2PGPDataType(stream: &mut ExiBitstream, PGPDataType: &ISO2PGPDataType )->Result<(), BitstreamError> {
      let mut grammar_id = 40;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           40=>{
               // Grammar: ID=40; read/write bits=2; START (PGPKeyID), START (PGPKeyPacket)
              if PGPDataType.choice_1.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (PGPKeyID, base64Binary); next=41
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_1.PGPKeyID.len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_1.PGPKeyID)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 41;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (PGPKeyPacket, base64Binary); next=42
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_1.PGPKeyPacket.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_1.PGPKeyPacket.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 42;
  
  
  
              }
  }
           41=>{
               // Grammar: ID=41; read/write bits=3; START (PGPKeyPacket), START (ANY), END Element, START (ANY)
              if PGPDataType.choice_1.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (PGPKeyPacket, base64Binary); next=42
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_1.PGPKeyPacket.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_1.PGPKeyPacket.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 42;
  
  
  
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=1)
              //{
              // ***** //
              else if PGPDataType.choice_1.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (ANY, base64Binary); next=43
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_1.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_1.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 43;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           42=>{
               // Grammar: ID=42; read/write bits=3; START (ANY), END Element, END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if PGPDataType.choice_1.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (ANY, base64Binary); next=43
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_1.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_1.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 43;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           43=>{
               // Grammar: ID=43; read/write bits=1; START (PGPKeyPacket)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=44
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(PGPDataType.choice_2.PGPKeyPacket.len() as u16)?;
                          stream.write_bytes( &PGPDataType.choice_2.PGPKeyPacket)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 44;
  
  
  }
           44=>{
               // Grammar: ID=44; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if PGPDataType.choice_2.is_some()
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (ANY, base64Binary); next=43
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_2.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_2.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 43;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SPKIData; type={http://www.w3.org/2000/09/xmldsig#}SPKIDataType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SPKISexp, base64Binary (1, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2SPKIDataType(stream: &mut ExiBitstream, SPKIDataType: &ISO2SPKIDataType )->Result<(), BitstreamError> {
      let mut grammar_id = 45;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           45=>{
               // Grammar: ID=45; read/write bits=1; START (SPKISexp)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=46
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(SPKIDataType.SPKISexp.len() as u16)?;
                          stream.write_bytes( &SPKIDataType.SPKISexp)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 46;
  
  
  }
           46=>{
               // Grammar: ID=46; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if SPKIDataType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SPKIDataType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &SPKIDataType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignedInfo; type={http://www.w3.org/2000/09/xmldsig#}SignedInfoType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); CanonicalizationMethod, CanonicalizationMethodType (1, 1); SignatureMethod, SignatureMethodType (1, 1); Reference, ReferenceType (1, 4);
  pub fn encode_ISO2SignedInfoType(stream: &mut ExiBitstream, SignedInfoType: &ISO2SignedInfoType )->Result<(), BitstreamError> {
      let mut grammar_id = 47;
      let mut done = false;
      let mut Reference_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           47=>{
               // Grammar: ID=47; read/write bits=2; START (Id), START (CanonicalizationMethod)
              if SignedInfoType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Id, NCName); next=48
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(SignedInfoType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&SignedInfoType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 48;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (CanonicalizationMethod, CanonicalizationMethodType); next=49
  
  
  
                      encode_ISO2CanonicalizationMethodType(stream,&SignedInfoType.CanonicalizationMethod)?;
  
                      grammar_id = 49;
              }
  }
           48=>{
               // Grammar: ID=48; read/write bits=1; START (CanonicalizationMethod)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (CanonicalizationMethodType); next=49
  
  
  
                  encode_ISO2CanonicalizationMethodType(stream,&SignedInfoType.CanonicalizationMethod)?;
  
                  grammar_id = 49;
  }
           49=>{
               // Grammar: ID=49; read/write bits=1; START (SignatureMethod)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (SignatureMethodType); next=50
  
  
  
                  encode_ISO2SignatureMethodType(stream,&SignedInfoType.SignatureMethod)?;
  
                  grammar_id = 50;
  }
           50=>{
               // Grammar: ID=50; read/write bits=1; START (Reference)
              if (Reference_currentIndex < SignedInfoType.Reference.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (ReferenceType); next=51
  
  
                      encode_ISO2ReferenceType(stream, &SignedInfoType.Reference[Reference_currentIndex])?;
                      Reference_currentIndex+=1;
                      grammar_id = 51;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           51=>{
               // Grammar: ID=51; read/write bits=2; START (Reference), END Element
              if (Reference_currentIndex < SignedInfoType.Reference.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ReferenceType); next=52
  
  
                      encode_ISO2ReferenceType(stream, &SignedInfoType.Reference[Reference_currentIndex])?;
                      Reference_currentIndex+=1;
                      grammar_id = 52;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           52=>{
               // Grammar: ID=52; read/write bits=2; START (Reference), END Element
              if (Reference_currentIndex < SignedInfoType.Reference.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ReferenceType); next=53
  
  
                      encode_ISO2ReferenceType(stream, &SignedInfoType.Reference[Reference_currentIndex])?;
                      Reference_currentIndex+=1;
                      grammar_id = 53;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           53=>{
               // Grammar: ID=53; read/write bits=2; START (Reference), END Element
              if (Reference_currentIndex < SignedInfoType.Reference.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ReferenceType); next=54
  
  
                      encode_ISO2ReferenceType(stream, &SignedInfoType.Reference[Reference_currentIndex])?;
                      Reference_currentIndex+=1;
                      grammar_id = 54;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           54=>{
               // Grammar: ID=54; read/write bits=2; START (Reference), END Element
              if (Reference_currentIndex < SignedInfoType.Reference.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ReferenceType); next=3
  
  
                      encode_ISO2ReferenceType(stream, &SignedInfoType.Reference[Reference_currentIndex])?;
                      Reference_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Service; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ServiceID, serviceIDType (1, 1); ServiceName, serviceNameType (0, 1); ServiceCategory, serviceCategoryType (1, 1); ServiceScope, serviceScopeType (0, 1); FreeService, boolean (1, 1);
  pub fn encode_ISO2ServiceType(stream: &mut ExiBitstream, ServiceType: &ISO2ServiceType )->Result<(), BitstreamError> {
      let mut grammar_id = 55;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           55=>{
               // Grammar: ID=55; read/write bits=1; START (ServiceID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedShort); next=56
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(ServiceType.ServiceID as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 56;
  
  
  }
           56=>{
               // Grammar: ID=56; read/write bits=2; START (ServiceName), START (ServiceCategory)
              if ServiceType.ServiceName.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ServiceName, string); next=57
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(ServiceType.ServiceName.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&ServiceType.ServiceName.expect("Value not Initialized").to_string(), ISO2ServiceName_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 57;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (ServiceCategory, string); next=58
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(2, ServiceType.ServiceCategory as u32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 58;
              }
  }
           57=>{
               // Grammar: ID=57; read/write bits=1; START (ServiceCategory)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=58
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, ServiceType.ServiceCategory as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 58;
  }
           58=>{
               // Grammar: ID=58; read/write bits=2; START (ServiceScope), START (FreeService)
              if ServiceType.ServiceScope.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ServiceScope, string); next=59
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(ServiceType.ServiceScope.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&ServiceType.ServiceScope.expect("Value not Initialized").to_string(), ISO2ServiceScope_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 59;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (FreeService, boolean); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ServiceType.FreeService)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  
              }
  }
           59=>{
               // Grammar: ID=59; read/write bits=1; START (FreeService)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (boolean); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( ServiceType.FreeService)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SelectedService; type={urn:iso:15118:2:2013:MsgDataTypes}SelectedServiceType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ServiceID, serviceIDType (1, 1); ParameterSetID, short (0, 1);
  pub fn encode_ISO2SelectedServiceType(stream: &mut ExiBitstream, SelectedServiceType: &ISO2SelectedServiceType )->Result<(), BitstreamError> {
      let mut grammar_id = 60;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           60=>{
               // Grammar: ID=60; read/write bits=1; START (ServiceID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedShort); next=61
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(SelectedServiceType.ServiceID as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 61;
  
  
  }
           61=>{
               // Grammar: ID=61; read/write bits=2; START (ParameterSetID), END Element
              if SelectedServiceType.ParameterSetID.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ParameterSetID, int); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( SelectedServiceType.ParameterSetID as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEStatusType; base type=EVSEStatusType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); RCD, boolean (1, 1);
  pub fn encode_ISO2AC_EVSEStatusType(stream: &mut ExiBitstream, AC_EVSEStatusType: &ISO2AC_EVSEStatusType )->Result<(), BitstreamError> {
      let mut grammar_id = 62;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           62=>{
               // Grammar: ID=62; read/write bits=1; START (NotificationMaxDelay)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedInt); next=63
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(AC_EVSEStatusType.NotificationMaxDelay as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 63;
  
  
  }
           63=>{
               // Grammar: ID=63; read/write bits=1; START (EVSENotification)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=64
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, AC_EVSEStatusType.EVSENotification as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 64;
  }
           64=>{
               // Grammar: ID=64; read/write bits=1; START (RCD)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (boolean); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( AC_EVSEStatusType.RCD)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignatureValue; type={http://www.w3.org/2000/09/xmldsig#}SignatureValueType; base type=base64Binary; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (0, 1); CONTENT, SignatureValueType (1, 1);
  pub fn encode_ISO2SignatureValueType(stream: &mut ExiBitstream, SignatureValueType: &ISO2SignatureValueType )->Result<(), BitstreamError> {
      let mut grammar_id = 65;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           65=>{
               // Grammar: ID=65; read/write bits=2; START (Id), START (CONTENT)
              if SignatureValueType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Id, NCName); next=66
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(SignatureValueType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&SignatureValueType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 66;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (CONTENT, base64Binary); next=3
                      stream.write_u16(SignatureValueType.CONTENT.len() as u16)?;
                      stream.write_bytes(&SignatureValueType.CONTENT)?;
                      grammar_id = 3;
  
              }
  }
           66=>{
               // Grammar: ID=66; read/write bits=1; START (CONTENT)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=3
                  stream.write_u16(SignatureValueType.CONTENT.len() as u16)?;
                  stream.write_bytes(&SignatureValueType.CONTENT)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SubCertificates; type={urn:iso:15118:2:2013:MsgDataTypes}SubCertificatesType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Certificate, certificateType (1, 4);
  pub fn encode_ISO2SubCertificatesType(stream: &mut ExiBitstream, SubCertificatesType: &ISO2SubCertificatesType )->Result<(), BitstreamError> {
      let mut grammar_id = 67;
      let mut done = false;
      let mut Certificate_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           67=>{
               // Grammar: ID=67; read/write bits=1; START (Certificate)
              if (Certificate_currentIndex < SubCertificatesType.Certificate.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (base64Binary); next=68
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SubCertificatesType.Certificate[Certificate_currentIndex].len() as u16)?;
                              stream.write_bytes( &SubCertificatesType.Certificate[Certificate_currentIndex])?;
                                  Certificate_currentIndex+=1;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 68;
  
  
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           68=>{
               // Grammar: ID=68; read/write bits=2; START (Certificate), END Element
              if (Certificate_currentIndex < SubCertificatesType.Certificate.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (base64Binary); next=69
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SubCertificatesType.Certificate[Certificate_currentIndex].len() as u16)?;
                              stream.write_bytes( &SubCertificatesType.Certificate[Certificate_currentIndex])?;
                                  Certificate_currentIndex+=1;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 69;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           69=>{
               // Grammar: ID=69; read/write bits=2; START (Certificate), END Element
              if (Certificate_currentIndex < SubCertificatesType.Certificate.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (base64Binary); next=70
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SubCertificatesType.Certificate[Certificate_currentIndex].len() as u16)?;
                              stream.write_bytes( &SubCertificatesType.Certificate[Certificate_currentIndex])?;
                                  Certificate_currentIndex+=1;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 70;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           70=>{
               // Grammar: ID=70; read/write bits=2; START (Certificate), END Element
              if (Certificate_currentIndex < SubCertificatesType.Certificate.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SubCertificatesType.Certificate[Certificate_currentIndex].len() as u16)?;
                              stream.write_bytes( &SubCertificatesType.Certificate[Certificate_currentIndex])?;
                                  Certificate_currentIndex+=1;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}KeyInfo; type={http://www.w3.org/2000/09/xmldsig#}KeyInfoType; base type=; content type=mixed;
  //          abstract=False; final=False; choice=True;
  // Particle: Id, ID (0, 1); KeyName, string (0, 1); KeyValue, KeyValueType (0, 1); RetrievalMethod, RetrievalMethodType (0, 1); X509Data, X509DataType (0, 1); PGPData, PGPDataType (0, 1); SPKIData, SPKIDataType (0, 1); MgmtData, string (0, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2KeyInfoType(stream: &mut ExiBitstream, KeyInfoType: &ISO2KeyInfoType )->Result<(), BitstreamError> {
      let mut grammar_id = 71;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           71=>{
               // Grammar: ID=71; read/write bits=4; START (Id), START (KeyName), START (KeyValue), START (RetrievalMethod), START (X509Data), START (PGPData), START (SPKIData), START (MgmtData), START (ANY)
              if KeyInfoType.Id.is_some()
              {
                  stream.write_nbit_uint(4, 0 as u32)?;
                  // Event: START (Id, NCName); next=72
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(KeyInfoType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&KeyInfoType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 72;
  
              }
              else if KeyInfoType.KeyName.is_some()
              {
                  stream.write_nbit_uint(4, 1 as u32)?;
                  // Event: START (KeyName, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(KeyInfoType.KeyName.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&KeyInfoType.KeyName.expect("Value not Initialized").to_string(), ISO2KeyName_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else if KeyInfoType.KeyValue.is_some()
              {
                  stream.write_nbit_uint(4, 2 as u32)?;
                  // Event: START (KeyValue, KeyValueType); next=3
  
  
   
                      encode_ISO2KeyValueType(stream,&KeyInfoType.KeyValue.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyInfoType.RetrievalMethod.is_some()
              {
                  stream.write_nbit_uint(4, 3 as u32)?;
                  // Event: START (RetrievalMethod, RetrievalMethodType); next=3
  
  
   
                      encode_ISO2RetrievalMethodType(stream,&KeyInfoType.RetrievalMethod.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyInfoType.X509Data.is_some()
              {
                  stream.write_nbit_uint(4, 4 as u32)?;
                  // Event: START (X509Data, X509DataType); next=3
  
  
   
                      encode_ISO2X509DataType(stream,&KeyInfoType.X509Data.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyInfoType.PGPData.is_some()
              {
                  stream.write_nbit_uint(4, 5 as u32)?;
                  // Event: START (PGPData, PGPDataType); next=3
  
  
   
                      encode_ISO2PGPDataType(stream,&KeyInfoType.PGPData.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyInfoType.SPKIData.is_some()
              {
                  stream.write_nbit_uint(4, 6 as u32)?;
                  // Event: START (SPKIData, SPKIDataType); next=3
  
  
   
                      encode_ISO2SPKIDataType(stream,&KeyInfoType.SPKIData.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyInfoType.MgmtData.is_some()
              {
                  stream.write_nbit_uint(4, 7 as u32)?;
                  // Event: START (MgmtData, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(KeyInfoType.MgmtData.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&KeyInfoType.MgmtData.expect("Value not Initialized").to_string(), ISO2MgmtData_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else if KeyInfoType.ANY.is_some()
              {
                  stream.write_nbit_uint(4, 8 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(KeyInfoType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &KeyInfoType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
  }
           72=>{
               // Grammar: ID=72; read/write bits=4; START (KeyName), START (KeyValue), START (RetrievalMethod), START (X509Data), START (PGPData), START (SPKIData), START (MgmtData), START (ANY)
              if KeyInfoType.KeyName.is_some()
              {
                  stream.write_nbit_uint(4, 0 as u32)?;
                  // Event: START (KeyName, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(KeyInfoType.KeyName.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&KeyInfoType.KeyName.expect("Value not Initialized").to_string(), ISO2KeyName_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else if KeyInfoType.KeyValue.is_some()
              {
                  stream.write_nbit_uint(4, 1 as u32)?;
                  // Event: START (KeyValue, KeyValueType); next=3
  
  
   
                      encode_ISO2KeyValueType(stream,&KeyInfoType.KeyValue.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyInfoType.RetrievalMethod.is_some()
              {
                  stream.write_nbit_uint(4, 2 as u32)?;
                  // Event: START (RetrievalMethod, RetrievalMethodType); next=3
  
  
   
                      encode_ISO2RetrievalMethodType(stream,&KeyInfoType.RetrievalMethod.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyInfoType.X509Data.is_some()
              {
                  stream.write_nbit_uint(4, 3 as u32)?;
                  // Event: START (X509Data, X509DataType); next=3
  
  
   
                      encode_ISO2X509DataType(stream,&KeyInfoType.X509Data.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyInfoType.PGPData.is_some()
              {
                  stream.write_nbit_uint(4, 4 as u32)?;
                  // Event: START (PGPData, PGPDataType); next=3
  
  
   
                      encode_ISO2PGPDataType(stream,&KeyInfoType.PGPData.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyInfoType.SPKIData.is_some()
              {
                  stream.write_nbit_uint(4, 5 as u32)?;
                  // Event: START (SPKIData, SPKIDataType); next=3
  
  
   
                      encode_ISO2SPKIDataType(stream,&KeyInfoType.SPKIData.unwrap())?;
  
                      grammar_id = 3;
              }
              else if KeyInfoType.MgmtData.is_some()
              {
                  stream.write_nbit_uint(4, 6 as u32)?;
                  // Event: START (MgmtData, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(KeyInfoType.MgmtData.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&KeyInfoType.MgmtData.expect("Value not Initialized").to_string(), ISO2MgmtData_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else if KeyInfoType.ANY.is_some()
              {
                  stream.write_nbit_uint(4, 7 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(KeyInfoType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &KeyInfoType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Object; type={http://www.w3.org/2000/09/xmldsig#}ObjectType; base type=; content type=mixed;
  //          abstract=False; final=False;
  // Particle: Encoding, anyURI (0, 1); Id, ID (0, 1); MimeType, string (0, 1); ANY, anyType (0, 1)(old 1, 1);
  pub fn encode_ISO2ObjectType(stream: &mut ExiBitstream, ObjectType: &ISO2ObjectType )->Result<(), BitstreamError> {
      let mut grammar_id = 73;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           73=>{
               // Grammar: ID=73; read/write bits=3; START (Encoding), START (Id), START (MimeType), START (ANY), END Element, START (ANY)
              if ObjectType.Encoding.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (Encoding, anyURI); next=74
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ObjectType.Encoding.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ObjectType.Encoding.expect("Value not Initialized").to_string(), ISO2Encoding_CHARACTER_SIZE)?;
  
                  grammar_id = 74;
  
              }
              else if ObjectType.Id.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (Id, NCName); next=75
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ObjectType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ObjectType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 75;
  
              }
              else if ObjectType.MimeType.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (MimeType, string); next=76
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ObjectType.MimeType.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ObjectType.MimeType.expect("Value not Initialized").to_string(), ISO2MimeType_CHARACTER_SIZE)?;
  
                  grammar_id = 76;
  
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=3)
              //{
              // ***** //
              else if ObjectType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 5 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(ObjectType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &ObjectType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 4 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           74=>{
               // Grammar: ID=74; read/write bits=3; START (Id), START (MimeType), START (ANY), END Element, START (ANY)
              if ObjectType.Id.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (Id, NCName); next=75
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ObjectType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ObjectType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 75;
  
              }
              else if ObjectType.MimeType.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (MimeType, string); next=76
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ObjectType.MimeType.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ObjectType.MimeType.expect("Value not Initialized").to_string(), ISO2MimeType_CHARACTER_SIZE)?;
  
                  grammar_id = 76;
  
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=2)
              //{
              // ***** //
              else if ObjectType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 4 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(ObjectType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &ObjectType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           75=>{
               // Grammar: ID=75; read/write bits=3; START (MimeType), START (ANY), END Element, START (ANY)
              if ObjectType.MimeType.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (MimeType, string); next=76
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ObjectType.MimeType.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&ObjectType.MimeType.expect("Value not Initialized").to_string(), ISO2MimeType_CHARACTER_SIZE)?;
  
                  grammar_id = 76;
  
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=1)
              //{
              // ***** //
              else if ObjectType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(ObjectType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &ObjectType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           76=>{
               // Grammar: ID=76; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if ObjectType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(ObjectType.ANY.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &ObjectType.ANY.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SupportedEnergyTransferMode; type={urn:iso:15118:2:2013:MsgDataTypes}SupportedEnergyTransferModeType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: EnergyTransferMode, EnergyTransferModeType (1, 6);
  pub fn encode_ISO2SupportedEnergyTransferModeType(stream: &mut ExiBitstream, SupportedEnergyTransferModeType: &ISO2SupportedEnergyTransferModeType )->Result<(), BitstreamError> {
      let mut grammar_id = 77;
      let mut done = false;
      let mut EnergyTransferMode_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           77=>{
               // Grammar: ID=77; read/write bits=1; START (EnergyTransferMode)
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (string); next=78
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode[EnergyTransferMode_currentIndex] as u32)?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 78;
  
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           78=>{
               // Grammar: ID=78; read/write bits=2; START (EnergyTransferMode), END Element
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (string); next=79
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode[EnergyTransferMode_currentIndex] as u32)?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 79;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           79=>{
               // Grammar: ID=79; read/write bits=2; START (EnergyTransferMode), END Element
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (string); next=80
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode[EnergyTransferMode_currentIndex] as u32)?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 80;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           80=>{
               // Grammar: ID=80; read/write bits=2; START (EnergyTransferMode), END Element
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (string); next=81
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode[EnergyTransferMode_currentIndex] as u32)?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 81;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           81=>{
               // Grammar: ID=81; read/write bits=2; START (EnergyTransferMode), END Element
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (string); next=82
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode[EnergyTransferMode_currentIndex] as u32)?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 82;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           82=>{
               // Grammar: ID=82; read/write bits=2; START (EnergyTransferMode), END Element
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (string); next=3
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode[EnergyTransferMode_currentIndex] as u32)?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}DC_EVStatus; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVStatusType; base type=EVStatusType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: EVReady, boolean (1, 1); EVErrorCode, DC_EVErrorCodeType (1, 1); EVRESSSOC, percentValueType (1, 1);
  pub fn encode_ISO2DC_EVStatusType(stream: &mut ExiBitstream, DC_EVStatusType: &ISO2DC_EVStatusType )->Result<(), BitstreamError> {
      let mut grammar_id = 83;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           83=>{
               // Grammar: ID=83; read/write bits=1; START (EVReady)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (boolean); next=84
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( DC_EVStatusType.EVReady)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 84;
  
  
  }
           84=>{
               // Grammar: ID=84; read/write bits=1; START (EVErrorCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=85
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(4, DC_EVStatusType.EVErrorCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 85;
  }
           85=>{
               // Grammar: ID=85; read/write bits=1; START (EVRESSSOC)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (byte); next=3
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(7, DC_EVStatusType.EVRESSSOC as u32)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
  
                  grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ContractSignatureCertChain; type={urn:iso:15118:2:2013:MsgDataTypes}CertificateChainType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); Certificate, certificateType (1, 1); SubCertificates, SubCertificatesType (0, 1);
  pub fn encode_ISO2CertificateChainType(stream: &mut ExiBitstream, CertificateChainType: &ISO2CertificateChainType )->Result<(), BitstreamError> {
      let mut grammar_id = 86;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           86=>{
               // Grammar: ID=86; read/write bits=2; START (Id), START (Certificate)
              if CertificateChainType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Id, NCName); next=87
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(CertificateChainType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&CertificateChainType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 87;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (Certificate, base64Binary); next=88
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(CertificateChainType.Certificate.len() as u16)?;
                              stream.write_bytes( &CertificateChainType.Certificate)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 88;
  
  
  
              }
  }
           87=>{
               // Grammar: ID=87; read/write bits=1; START (Certificate)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=88
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(CertificateChainType.Certificate.len() as u16)?;
                          stream.write_bytes( &CertificateChainType.Certificate)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 88;
  
  
  }
           88=>{
               // Grammar: ID=88; read/write bits=2; START (SubCertificates), END Element
              if CertificateChainType.SubCertificates.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (SubCertificates, SubCertificatesType); next=3
  
  
   
                      encode_ISO2SubCertificatesType(stream,&CertificateChainType.SubCertificates.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}BodyElement; type={urn:iso:15118:2:2013:MsgBody}BodyBaseType; base type=; content type=empty;
  //          abstract=True; final=False;
  fn encode_ISO2BodyBaseType(stream: &mut ExiBitstream,BodyBaseType: &ISO2BodyBaseType)->Result<(),BitstreamError>{
      // Element has no particles, so the function just encodes END Element
      let _ = BodyBaseType;// To silence the unused variable warning
  
  //    error:i32= exi_basetypes_encoder_nbit_uint(stream, 1, 0);
      stream.write_nbit_uint(1,0)?;
       Ok(())
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgHeader}Notification; type={urn:iso:15118:2:2013:MsgDataTypes}NotificationType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: FaultCode, faultCodeType (1, 1); FaultMsg, faultMsgType (0, 1);
  pub fn encode_ISO2NotificationType(stream: &mut ExiBitstream, NotificationType: &ISO2NotificationType )->Result<(), BitstreamError> {
      let mut grammar_id = 89;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           89=>{
               // Grammar: ID=89; read/write bits=1; START (FaultCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=90
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, NotificationType.FaultCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 90;
  }
           90=>{
               // Grammar: ID=90; read/write bits=2; START (FaultMsg), END Element
              if NotificationType.FaultMsg.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (FaultMsg, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(NotificationType.FaultMsg.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&NotificationType.FaultMsg.expect("Value not Initialized").to_string(), ISO2FaultMsg_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentOptionList; type={urn:iso:15118:2:2013:MsgDataTypes}PaymentOptionListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: PaymentOption, paymentOptionType (1, 2);
  pub fn encode_ISO2PaymentOptionListType(stream: &mut ExiBitstream, PaymentOptionListType: &ISO2PaymentOptionListType )->Result<(), BitstreamError> {
      let mut grammar_id = 91;
      let mut done = false;
      let mut PaymentOption_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           91=>{
               // Grammar: ID=91; read/write bits=1; START (PaymentOption)
              if (PaymentOption_currentIndex < PaymentOptionListType.PaymentOption.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (string); next=92
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(1, PaymentOptionListType.PaymentOption[PaymentOption_currentIndex] as u32)?;
                      PaymentOption_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 92;
  
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           92=>{
               // Grammar: ID=92; read/write bits=2; START (PaymentOption), END Element
              if (PaymentOption_currentIndex < PaymentOptionListType.PaymentOption.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (string); next=3
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(1, PaymentOptionListType.PaymentOption[PaymentOption_currentIndex] as u32)?;
                      PaymentOption_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SelectedServiceList; type={urn:iso:15118:2:2013:MsgDataTypes}SelectedServiceListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SelectedService, SelectedServiceType (1, 16);
  pub fn encode_ISO2SelectedServiceListType(stream: &mut ExiBitstream, SelectedServiceListType: &ISO2SelectedServiceListType )->Result<(), BitstreamError> {
      let mut grammar_id = 93;
      let mut done = false;
      let mut SelectedService_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           93=>{
               // Grammar: ID=93; read/write bits=1; START (SelectedService)
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (SelectedServiceType); next=94
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 94;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           94=>{
               // Grammar: ID=94; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=95
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 95;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           95=>{
               // Grammar: ID=95; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=96
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 96;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           96=>{
               // Grammar: ID=96; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=97
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 97;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           97=>{
               // Grammar: ID=97; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=98
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 98;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           98=>{
               // Grammar: ID=98; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=99
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 99;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           99=>{
               // Grammar: ID=99; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=100
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 100;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           100=>{
               // Grammar: ID=100; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=101
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 101;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           101=>{
               // Grammar: ID=101; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=102
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 102;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           102=>{
               // Grammar: ID=102; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=103
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 103;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           103=>{
               // Grammar: ID=103; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=104
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 104;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           104=>{
               // Grammar: ID=104; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=105
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 105;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           105=>{
               // Grammar: ID=105; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=106
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 106;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           106=>{
               // Grammar: ID=106; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=107
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 107;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           107=>{
               // Grammar: ID=107; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=108
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 108;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           108=>{
               // Grammar: ID=108; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SelectedServiceType); next=3
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}DC_EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEStatusType; base type=EVSEStatusType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); EVSEIsolationStatus, isolationLevelType (0, 1); EVSEStatusCode, DC_EVSEStatusCodeType (1, 1);
  pub fn encode_ISO2DC_EVSEStatusType(stream: &mut ExiBitstream, DC_EVSEStatusType: &ISO2DC_EVSEStatusType )->Result<(), BitstreamError> {
      let mut grammar_id = 109;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           109=>{
               // Grammar: ID=109; read/write bits=1; START (NotificationMaxDelay)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedInt); next=110
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(DC_EVSEStatusType.NotificationMaxDelay as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 110;
  
  
  }
           110=>{
               // Grammar: ID=110; read/write bits=1; START (EVSENotification)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=111
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, DC_EVSEStatusType.EVSENotification as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 111;
  }
           111=>{
               // Grammar: ID=111; read/write bits=2; START (EVSEIsolationStatus), START (EVSEStatusCode)
              if DC_EVSEStatusType.EVSEIsolationStatus.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (EVSEIsolationStatus, string); next=112
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(3, DC_EVSEStatusType.EVSEIsolationStatus as u32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 112;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (EVSEStatusCode, string); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(4, DC_EVSEStatusType.EVSEStatusCode as u32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
              }
  }
           112=>{
               // Grammar: ID=112; read/write bits=1; START (EVSEStatusCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(4, DC_EVSEStatusType.EVSEStatusCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}EVSEStatusType; base type=; content type=ELEMENT-ONLY;
  //          abstract=True; final=False;
  // Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (1, 1);
  pub fn encode_ISO2EVSEStatusType(stream: &mut ExiBitstream, EVSEStatusType: &ISO2EVSEStatusType )->Result<(), BitstreamError> {
      let mut grammar_id = 113;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           113=>{
               // Grammar: ID=113; read/write bits=1; START (NotificationMaxDelay)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedInt); next=114
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(EVSEStatusType.NotificationMaxDelay as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 114;
  
  
  }
           114=>{
               // Grammar: ID=114; read/write bits=1; START (EVSENotification)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=115
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, EVSEStatusType.EVSENotification as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 115;
  }
           115=>{
               // Grammar: ID=115; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVSEStatusType); next=116
  
  
  
                  encode_ISO2DC_EVSEStatusType(stream,&EVSEStatusType.DC_EVSEStatus)?;
  
                  grammar_id = 116;
  }
           116=>{
               // Grammar: ID=116; read/write bits=1; START (AC_EVSEStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVSEStatusType); next=3
  
  
  
                  encode_ISO2AC_EVSEStatusType(stream,&EVSEStatusType.AC_EVSEStatus)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Signature; type={http://www.w3.org/2000/09/xmldsig#}SignatureType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); SignedInfo, SignedInfoType (1, 1); SignatureValue, SignatureValueType (1, 1); KeyInfo, KeyInfoType (0, 1); Object, ObjectType (0, 1);
  pub fn encode_ISO2SignatureType(stream: &mut ExiBitstream, SignatureType: &ISO2SignatureType )->Result<(), BitstreamError> {
      let mut grammar_id = 117;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           117=>{
               // Grammar: ID=117; read/write bits=2; START (Id), START (SignedInfo)
              if SignatureType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Id, NCName); next=118
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(SignatureType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&SignatureType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 118;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (SignedInfo, SignedInfoType); next=119
  
  
  
                      encode_ISO2SignedInfoType(stream,&SignatureType.SignedInfo)?;
  
                      grammar_id = 119;
              }
  }
           118=>{
               // Grammar: ID=118; read/write bits=1; START (SignedInfo)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (SignedInfoType); next=119
  
  
  
                  encode_ISO2SignedInfoType(stream,&SignatureType.SignedInfo)?;
  
                  grammar_id = 119;
  }
           119=>{
               // Grammar: ID=119; read/write bits=1; START (SignatureValue)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=120
  
  
  
                  encode_ISO2SignatureValueType(stream,&SignatureType.SignatureValue)?;
  
                  grammar_id = 120;
  }
           120=>{
               // Grammar: ID=120; read/write bits=2; START (KeyInfo), START (Object), END Element
              if SignatureType.KeyInfo.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (KeyInfo, KeyInfoType); next=122
  
  
   
                      encode_ISO2KeyInfoType(stream,&SignatureType.KeyInfo.unwrap())?;
  
                      grammar_id = 122;
              }
              else if SignatureType.Object.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (Object, ObjectType); next=121
  
  
   
                      encode_ISO2ObjectType(stream,&SignatureType.Object.unwrap())?;
  
                      grammar_id = 121;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           121=>{
               // Grammar: ID=121; read/write bits=2; START (Object), END Element
              if (1 == 0)
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Object, ObjectType); next=3
  
  
  
                      encode_ISO2ObjectType(stream,&SignatureType.Object)?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           122=>{
               // Grammar: ID=122; read/write bits=2; START (Object), END Element
              if SignatureType.Object.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Object, ObjectType); next=123
  
  
   
                      encode_ISO2ObjectType(stream,&SignatureType.Object.unwrap())?;
  
                      grammar_id = 123;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           123=>{
               // Grammar: ID=123; read/write bits=2; START (Object), END Element
              if (1 == 0)
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Object, ObjectType); next=3
  
  
  
                      encode_ISO2ObjectType(stream,&SignatureType.Object)?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeService; type={urn:iso:15118:2:2013:MsgDataTypes}ChargeServiceType; base type=ServiceType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ServiceID, serviceIDType (1, 1); ServiceName, serviceNameType (0, 1); ServiceCategory, serviceCategoryType (1, 1); ServiceScope, serviceScopeType (0, 1); FreeService, boolean (1, 1); SupportedEnergyTransferMode, SupportedEnergyTransferModeType (1, 1);
  pub fn encode_ISO2ChargeServiceType(stream: &mut ExiBitstream, ChargeServiceType: &ISO2ChargeServiceType )->Result<(), BitstreamError> {
      let mut grammar_id = 124;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           124=>{
               // Grammar: ID=124; read/write bits=1; START (ServiceID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedShort); next=125
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(ChargeServiceType.ServiceID as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 125;
  
  
  }
           125=>{
               // Grammar: ID=125; read/write bits=2; START (ServiceName), START (ServiceCategory)
              if ChargeServiceType.ServiceName.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ServiceName, string); next=126
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(ChargeServiceType.ServiceName.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&ChargeServiceType.ServiceName.expect("Value not Initialized").to_string(), ISO2ServiceName_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 126;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (ServiceCategory, string); next=127
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(2, ChargeServiceType.ServiceCategory as u32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 127;
              }
  }
           126=>{
               // Grammar: ID=126; read/write bits=1; START (ServiceCategory)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=127
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, ChargeServiceType.ServiceCategory as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 127;
  }
           127=>{
               // Grammar: ID=127; read/write bits=2; START (ServiceScope), START (FreeService)
              if ChargeServiceType.ServiceScope.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ServiceScope, string); next=128
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(ChargeServiceType.ServiceScope.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&ChargeServiceType.ServiceScope.expect("Value not Initialized").to_string(), ISO2ServiceScope_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 128;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (FreeService, boolean); next=129
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ChargeServiceType.FreeService)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 129;
  
  
  
              }
  }
           128=>{
               // Grammar: ID=128; read/write bits=1; START (FreeService)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (boolean); next=129
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( ChargeServiceType.FreeService)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 129;
  
  
  }
           129=>{
               // Grammar: ID=129; read/write bits=1; START (SupportedEnergyTransferMode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (SupportedEnergyTransferModeType); next=3
  
  
  
                  encode_ISO2SupportedEnergyTransferModeType(stream,&ChargeServiceType.SupportedEnergyTransferMode)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}EVSEPresentVoltage; type={urn:iso:15118:2:2013:MsgDataTypes}PhysicalValueType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Multiplier, unitMultiplierType (1, 1); Unit, unitSymbolType (1, 1); Value, short (1, 1);
  pub fn encode_ISO2PhysicalValueType(stream: &mut ExiBitstream, PhysicalValueType: &ISO2PhysicalValueType )->Result<(), BitstreamError> {
      let mut grammar_id = 130;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           130=>{
               // Grammar: ID=130; read/write bits=1; START (Multiplier)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (byte); next=131
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(3, PhysicalValueType.Multiplier as u32 + 3)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
  
                  grammar_id = 131;
  
  
  }
           131=>{
               // Grammar: ID=131; read/write bits=1; START (Unit)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=132
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(3, PhysicalValueType.Unit as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 132;
  }
           132=>{
               // Grammar: ID=132; read/write bits=1; START (Value)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (int); next=3
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_i16( PhysicalValueType.Value as i16)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ConsumptionCost; type={urn:iso:15118:2:2013:MsgDataTypes}ConsumptionCostType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: startValue, PhysicalValueType (1, 1); Cost, CostType (1, 3);
  pub fn encode_ISO2ConsumptionCostType(stream: &mut ExiBitstream, ConsumptionCostType: &ISO2ConsumptionCostType )->Result<(), BitstreamError> {
      let mut grammar_id = 133;
      let mut done = false;
      let mut Cost_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           133=>{
               // Grammar: ID=133; read/write bits=1; START (startValue)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=134
  
  
  
                  encode_ISO2PhysicalValueType(stream,&ConsumptionCostType.startValue)?;
  
                  grammar_id = 134;
  }
           134=>{
               // Grammar: ID=134; read/write bits=1; START (Cost)
              if (Cost_currentIndex < ConsumptionCostType.Cost.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (CostType); next=135
  
  
                      encode_ISO2CostType(stream, &ConsumptionCostType.Cost[Cost_currentIndex])?;
                      Cost_currentIndex+=1;
                      grammar_id = 135;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           135=>{
               // Grammar: ID=135; read/write bits=2; START (Cost), END Element
              if (Cost_currentIndex < ConsumptionCostType.Cost.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (CostType); next=136
  
  
                      encode_ISO2CostType(stream, &ConsumptionCostType.Cost[Cost_currentIndex])?;
                      Cost_currentIndex+=1;
                      grammar_id = 136;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           136=>{
               // Grammar: ID=136; read/write bits=2; START (Cost), END Element
              if (Cost_currentIndex < ConsumptionCostType.Cost.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (CostType); next=3
  
  
                      encode_ISO2CostType(stream, &ConsumptionCostType.Cost[Cost_currentIndex])?;
                      Cost_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleEntry; type={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleEntryType; base type=EntryType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: RelativeTimeInterval, RelativeTimeIntervalType (0, 1); TimeInterval, IntervalType (0, 1); PMax, PhysicalValueType (1, 1);
  pub fn encode_ISO2PMaxScheduleEntryType(stream: &mut ExiBitstream, PMaxScheduleEntryType: &ISO2PMaxScheduleEntryType )->Result<(), BitstreamError> {
      let mut grammar_id = 137;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           137=>{
               // Grammar: ID=137; read/write bits=2; START (RelativeTimeInterval), START (TimeInterval)
              if PMaxScheduleEntryType.RelativeTimeInterval.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (RelativeTimeInterval, IntervalType); next=138
  
  
   
                      encode_ISO2RelativeTimeIntervalType(stream,&PMaxScheduleEntryType.RelativeTimeInterval.unwrap())?;
  
                      grammar_id = 138;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Abstract element or type: START (IntervalType); next=138
  
  
  
                      encode_ISO2IntervalType(stream,&PMaxScheduleEntryType.TimeInterval)?;
  
                      grammar_id = 138;
              }
  }
           138=>{
               // Grammar: ID=138; read/write bits=1; START (PMax)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=3
  
  
  
                  encode_ISO2PhysicalValueType(stream,&PMaxScheduleEntryType.PMax)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffEntry; type={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffEntryType; base type=EntryType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: RelativeTimeInterval, RelativeTimeIntervalType (0, 1); TimeInterval, IntervalType (0, 1); EPriceLevel, unsignedByte (0, 1); ConsumptionCost, ConsumptionCostType (0, 3);
  pub fn encode_ISO2SalesTariffEntryType(stream: &mut ExiBitstream, SalesTariffEntryType: &ISO2SalesTariffEntryType )->Result<(), BitstreamError> {
      let mut grammar_id = 139;
      let mut done = false;
      let mut ConsumptionCost_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           139=>{
               // Grammar: ID=139; read/write bits=2; START (RelativeTimeInterval), START (TimeInterval)
              if SalesTariffEntryType.RelativeTimeInterval.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (RelativeTimeInterval, IntervalType); next=140
  
  
   
                      encode_ISO2RelativeTimeIntervalType(stream,&SalesTariffEntryType.RelativeTimeInterval.unwrap())?;
  
                      grammar_id = 140;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Abstract element or type: START (IntervalType); next=140
  
  
  
                      encode_ISO2IntervalType(stream,&SalesTariffEntryType.TimeInterval)?;
  
                      grammar_id = 140;
              }
  }
           140=>{
               // Grammar: ID=140; read/write bits=2; START (EPriceLevel), START (ConsumptionCost), END Element
              if SalesTariffEntryType.EPriceLevel.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (EPriceLevel, unsignedShort); next=143
  
  
                      stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint( 8, SalesTariffEntryType.EPriceLevel as u32) ?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 143;
  
  
  
              }
              else if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.len())
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=141 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 141;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           141=>{
               // Grammar: ID=141; read/write bits=2; START (ConsumptionCost), END Element
              if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.len())
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=142 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 142;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           142=>{
               // Grammar: ID=142; read/write bits=2; START (ConsumptionCost), END Element
              if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.len())
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=3 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           143=>{
               // Grammar: ID=143; read/write bits=2; START (ConsumptionCost), END Element
              if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.len())
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=144 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 144;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           144=>{
               // Grammar: ID=144; read/write bits=2; START (ConsumptionCost), END Element
              if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.len())
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=145 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 145;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           145=>{
               // Grammar: ID=145; read/write bits=2; START (ConsumptionCost), END Element
              if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.len())
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=3 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Parameter; type={urn:iso:15118:2:2013:MsgDataTypes}ParameterType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; choice=True;
  // Particle: Name, string (1, 1); boolValue, boolean (0, 1); byteValue, byte (0, 1); shortValue, short (0, 1); intValue, int (0, 1); physicalValue, PhysicalValueType (0, 1); stringValue, string (0, 1);
  pub fn encode_ISO2ParameterType(stream: &mut ExiBitstream, ParameterType: &ISO2ParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 146;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           146=>{
               // Grammar: ID=146; read/write bits=1; START (Name)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=147
  
              // string should not be found in table, so add 2
  
               stream.write_u16(ParameterType.Name.len() as u16)?;
              stream.write_characters(&ParameterType.Name.to_string(), ISO2Name_CHARACTER_SIZE)?;
  
              grammar_id = 147;
  }
           147=>{
               // Grammar: ID=147; read/write bits=3; START (boolValue), START (byteValue), START (shortValue), START (intValue), START (physicalValue), START (stringValue)
              if ParameterType.boolValue.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (boolValue, boolean); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ParameterType.boolValue)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  
              }
              else if ParameterType.byteValue.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (byteValue, short); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                          // type has min_value = -128
                          stream.write_nbit_uint( 8, ParameterType.byteValue + -128)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                          grammar_id = 3;
  
  
  
              }
              else if ParameterType.shortValue.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (shortValue, int); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( ParameterType.shortValue as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
  
              }
              else if ParameterType.intValue.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (intValue, long); next=3
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_i32(ParameterType.intValue as write_i32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
              }
              else if ParameterType.physicalValue.is_some()
              {
                  stream.write_nbit_uint(3, 4 as u32)?;
                  // Event: START (physicalValue, PhysicalValueType); next=3
  
  
   
                      encode_ISO2PhysicalValueType(stream,&ParameterType.physicalValue.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(3, 5 as u32)?;
                  // Event: START (stringValue, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(ParameterType.stringValue.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&ParameterType.stringValue.expect("Value not Initialized").to_string(), ISO2stringValue_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}PMaxSchedule; type={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: PMaxScheduleEntry, PMaxScheduleEntryType (1, 1024);
  pub fn encode_ISO2PMaxScheduleType(stream: &mut ExiBitstream, PMaxScheduleType: &ISO2PMaxScheduleType )->Result<(), BitstreamError> {
      let mut grammar_id = 148;
      let mut done = false;
      let mut PMaxScheduleEntry_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           148=>{
               // Grammar: ID=148; read/write bits=1; START (PMaxScheduleEntry)
              if (PMaxScheduleEntry_currentIndex < PMaxScheduleType.PMaxScheduleEntry.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (EntryType); next=149
  
  
                      encode_ISO2PMaxScheduleEntryType(stream, &PMaxScheduleType.PMaxScheduleEntry[PMaxScheduleEntry_currentIndex])?;
                      PMaxScheduleEntry_currentIndex+=1;
                      grammar_id = 149;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           149=>{
               // Grammar: ID=149; read/write bits=2; LOOP (PMaxScheduleEntry), END Element
              if (PMaxScheduleEntry_currentIndex < PMaxScheduleType.PMaxScheduleEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: LOOP (EntryType); next=4
  
  
                      encode_ISO2PMaxScheduleEntryType(stream, &PMaxScheduleType.PMaxScheduleEntry[PMaxScheduleEntry_currentIndex])?;
                      PMaxScheduleEntry_currentIndex+=1;
                      grammar_id = 4;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SalesTariff; type={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); SalesTariffID, SAIDType (1, 1); SalesTariffDescription, tariffDescriptionType (0, 1); NumEPriceLevels, unsignedByte (0, 1); SalesTariffEntry, SalesTariffEntryType (1, 1024);
  pub fn encode_ISO2SalesTariffType(stream: &mut ExiBitstream, SalesTariffType: &ISO2SalesTariffType )->Result<(), BitstreamError> {
      let mut grammar_id = 150;
      let mut done = false;
      let mut SalesTariffEntry_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           150=>{
               // Grammar: ID=150; read/write bits=2; START (Id), START (SalesTariffID)
              if SalesTariffType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Id, NCName); next=151
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(SalesTariffType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&SalesTariffType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 151;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (SalesTariffID, unsignedByte); next=152
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(8, SalesTariffType.SalesTariffID as u32 - 1)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 152;
  
  
  
              }
  }
           151=>{
               // Grammar: ID=151; read/write bits=1; START (SalesTariffID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedByte); next=152
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(8, SalesTariffType.SalesTariffID as u32 - 1)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
  
                  grammar_id = 152;
  
  
  }
           152=>{
               // Grammar: ID=152; read/write bits=2; START (SalesTariffDescription), START (NumEPriceLevels), START (SalesTariffEntry)
              if SalesTariffType.SalesTariffDescription.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (SalesTariffDescription, string); next=154
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(SalesTariffType.SalesTariffDescription.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&SalesTariffType.SalesTariffDescription.expect("Value not Initialized").to_string(), ISO2SalesTariffDescription_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 154;
              }
              else if SalesTariffType.NumEPriceLevels.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (NumEPriceLevels, unsignedShort); next=156
  
  
                      stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint( 8, SalesTariffType.NumEPriceLevels as u32) ?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 156;
  
  
  
              }
              else
              {
                  if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
                  {
                      stream.write_nbit_uint(2,2 as u32)?;
                      // Event: START (EntryType); next=3
  
  
                          encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry[SalesTariffEntry_currentIndex])?;
                          SalesTariffEntry_currentIndex+=1;
                          grammar_id = 3;
  
                  }
              }
  }
           153=>{
               // Grammar: ID=153; read/write bits=2; LOOP (SalesTariffEntry), END Element
              if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: LOOP (EntryType); next=4
  
  
                      encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry[SalesTariffEntry_currentIndex])?;
                      SalesTariffEntry_currentIndex+=1;
                      grammar_id = 4;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           154=>{
               // Grammar: ID=154; read/write bits=2; START (NumEPriceLevels), START (SalesTariffEntry)
              if SalesTariffType.NumEPriceLevels.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (NumEPriceLevels, unsignedShort); next=156
  
  
                      stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint( 8, SalesTariffType.NumEPriceLevels as u32) ?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 156;
  
  
  
              }
              else
              {
                  if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
                  {
                      stream.write_nbit_uint(2,1 as u32)?;
                      // Event: START (EntryType); next=3
  
  
                          encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry[SalesTariffEntry_currentIndex])?;
                          SalesTariffEntry_currentIndex+=1;
                          grammar_id = 3;
  
                  }
              }
  }
           155=>{
               // Grammar: ID=155; read/write bits=2; LOOP (SalesTariffEntry), END Element
              if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: LOOP (EntryType); next=4
  
  
                      encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry[SalesTariffEntry_currentIndex])?;
                      SalesTariffEntry_currentIndex+=1;
                      grammar_id = 4;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           156=>{
               // Grammar: ID=156; read/write bits=1; START (SalesTariffEntry)
              if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (EntryType); next=157
  
  
                      encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry[SalesTariffEntry_currentIndex])?;
                      SalesTariffEntry_currentIndex+=1;
                      grammar_id = 157;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           157=>{
               // Grammar: ID=157; read/write bits=2; LOOP (SalesTariffEntry), END Element
              if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: LOOP (EntryType); next=4
  
  
                      encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry[SalesTariffEntry_currentIndex])?;
                      SalesTariffEntry_currentIndex+=1;
                      grammar_id = 4;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ParameterSet; type={urn:iso:15118:2:2013:MsgDataTypes}ParameterSetType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ParameterSetID, short (1, 1); Parameter, ParameterType (1, 16);
  pub fn encode_ISO2ParameterSetType(stream: &mut ExiBitstream, ParameterSetType: &ISO2ParameterSetType )->Result<(), BitstreamError> {
      let mut grammar_id = 158;
      let mut done = false;
      let mut Parameter_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           158=>{
               // Grammar: ID=158; read/write bits=1; START (ParameterSetID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (int); next=159
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_i16( ParameterSetType.ParameterSetID as i16)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 159;
  
  
  
  }
           159=>{
               // Grammar: ID=159; read/write bits=1; START (Parameter)
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (ParameterType); next=160
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 160;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           160=>{
               // Grammar: ID=160; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=161
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 161;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           161=>{
               // Grammar: ID=161; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=162
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 162;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           162=>{
               // Grammar: ID=162; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=163
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 163;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           163=>{
               // Grammar: ID=163; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=164
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 164;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           164=>{
               // Grammar: ID=164; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=165
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 165;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           165=>{
               // Grammar: ID=165; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=166
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 166;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           166=>{
               // Grammar: ID=166; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=167
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 167;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           167=>{
               // Grammar: ID=167; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=168
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 168;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           168=>{
               // Grammar: ID=168; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=169
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 169;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           169=>{
               // Grammar: ID=169; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=170
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 170;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           170=>{
               // Grammar: ID=170; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=171
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 171;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           171=>{
               // Grammar: ID=171; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=172
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 172;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           172=>{
               // Grammar: ID=172; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=173
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 173;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           173=>{
               // Grammar: ID=173; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=174
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 174;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           174=>{
               // Grammar: ID=174; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ParameterType); next=3
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleTuple; type={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleTupleType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SAScheduleTupleID, SAIDType (1, 1); PMaxSchedule, PMaxScheduleType (1, 1); SalesTariff, SalesTariffType (0, 1);
  pub fn encode_ISO2SAScheduleTupleType(stream: &mut ExiBitstream, SAScheduleTupleType: &ISO2SAScheduleTupleType )->Result<(), BitstreamError> {
      let mut grammar_id = 175;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           175=>{
               // Grammar: ID=175; read/write bits=1; START (SAScheduleTupleID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedByte); next=176
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(8, SAScheduleTupleType.SAScheduleTupleID as u32 - 1)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
  
                  grammar_id = 176;
  
  
  }
           176=>{
               // Grammar: ID=176; read/write bits=1; START (PMaxSchedule)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PMaxScheduleType); next=177
  
  
  
                  encode_ISO2PMaxScheduleType(stream,&SAScheduleTupleType.PMaxSchedule)?;
  
                  grammar_id = 177;
  }
           177=>{
               // Grammar: ID=177; read/write bits=2; START (SalesTariff), END Element
              if SAScheduleTupleType.SalesTariff.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (SalesTariff, SalesTariffType); next=3
  
  
   
                      encode_ISO2SalesTariffType(stream,&SAScheduleTupleType.SalesTariff.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ProfileEntry; type={urn:iso:15118:2:2013:MsgDataTypes}ProfileEntryType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ChargingProfileEntryStart, unsignedInt (1, 1); ChargingProfileEntryMaxPower, PhysicalValueType (1, 1); ChargingProfileEntryMaxNumberOfPhasesInUse, maxNumPhasesType (0, 1);
  pub fn encode_ISO2ProfileEntryType(stream: &mut ExiBitstream, ProfileEntryType: &ISO2ProfileEntryType )->Result<(), BitstreamError> {
      let mut grammar_id = 178;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           178=>{
               // Grammar: ID=178; read/write bits=1; START (ChargingProfileEntryStart)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedLong); next=179
  
  
                  stream.write_nbit_uint( 1, 0)?;
                  stream.write_u32(ProfileEntryType.ChargingProfileEntryStart as u32)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 179;
  
  
  }
           179=>{
               // Grammar: ID=179; read/write bits=1; START (ChargingProfileEntryMaxPower)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=180
  
  
  
                  encode_ISO2PhysicalValueType(stream,&ProfileEntryType.ChargingProfileEntryMaxPower)?;
  
                  grammar_id = 180;
  }
           180=>{
               // Grammar: ID=180; read/write bits=2; START (ChargingProfileEntryMaxNumberOfPhasesInUse), END Element
              if ProfileEntryType.ChargingProfileEntryMaxNumberOfPhasesInUse.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ChargingProfileEntryMaxNumberOfPhasesInUse, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?; 
  
                      stream.write_nbit_uint(2, ProfileEntryType.ChargingProfileEntryMaxNumberOfPhasesInUse.unwrap() as u32 - 1)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ListOfRootCertificateIDs; type={urn:iso:15118:2:2013:MsgDataTypes}ListOfRootCertificateIDsType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: RootCertificateID, X509IssuerSerialType (1, 20);
  pub fn encode_ISO2ListOfRootCertificateIDsType(stream: &mut ExiBitstream, ListOfRootCertificateIDsType: &ISO2ListOfRootCertificateIDsType )->Result<(), BitstreamError> {
      let mut grammar_id = 181;
      let mut done = false;
      let mut RootCertificateID_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           181=>{
               // Grammar: ID=181; read/write bits=1; START (RootCertificateID)
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=182
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 182;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           182=>{
               // Grammar: ID=182; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=183
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 183;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           183=>{
               // Grammar: ID=183; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=184
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 184;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           184=>{
               // Grammar: ID=184; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=185
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 185;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           185=>{
               // Grammar: ID=185; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=186
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 186;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           186=>{
               // Grammar: ID=186; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=187
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 187;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           187=>{
               // Grammar: ID=187; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=188
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 188;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           188=>{
               // Grammar: ID=188; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=189
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 189;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           189=>{
               // Grammar: ID=189; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=190
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 190;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           190=>{
               // Grammar: ID=190; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=191
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 191;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           191=>{
               // Grammar: ID=191; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=192
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 192;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           192=>{
               // Grammar: ID=192; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=193
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 193;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           193=>{
               // Grammar: ID=193; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=194
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 194;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           194=>{
               // Grammar: ID=194; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=195
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 195;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           195=>{
               // Grammar: ID=195; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=196
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 196;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           196=>{
               // Grammar: ID=196; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=197
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 197;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           197=>{
               // Grammar: ID=197; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=198
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 198;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           198=>{
               // Grammar: ID=198; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=199
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 199;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           199=>{
               // Grammar: ID=199; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=200
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 200;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           200=>{
               // Grammar: ID=200; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (X509IssuerSerialType); next=3
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceParameterList; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceParameterListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ParameterSet, ParameterSetType (1, 255);
  pub fn encode_ISO2ServiceParameterListType(stream: &mut ExiBitstream, ServiceParameterListType: &ISO2ServiceParameterListType )->Result<(), BitstreamError> {
      let mut grammar_id = 201;
      let mut done = false;
      let mut ParameterSet_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           201=>{
               // Grammar: ID=201; read/write bits=1; START (ParameterSet)
              if (ParameterSet_currentIndex < ServiceParameterListType.ParameterSet.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (ParameterSetType); next=202
  
  
                      encode_ISO2ParameterSetType(stream, &ServiceParameterListType.ParameterSet[ParameterSet_currentIndex])?;
                      ParameterSet_currentIndex+=1;
                      grammar_id = 202;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           202=>{
               // Grammar: ID=202; read/write bits=2; LOOP (ParameterSet), END Element
              if (ParameterSet_currentIndex < ServiceParameterListType.ParameterSet.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: LOOP (ParameterSetType); next=4
  
  
                      encode_ISO2ParameterSetType(stream, &ServiceParameterListType.ParameterSet[ParameterSet_currentIndex])?;
                      ParameterSet_currentIndex+=1;
                      grammar_id = 4;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SASchedules; type={urn:iso:15118:2:2013:MsgDataTypes}SASchedulesType; base type=; content type=empty;
  //          abstract=True; final=False;
  fn encode_ISO2SASchedulesType(stream: &mut ExiBitstream,SASchedulesType: &ISO2SASchedulesType)->Result<(),BitstreamError>{
      // Element has no particles, so the function just encodes END Element
      let _ = SASchedulesType;// To silence the unused variable warning
  
  //    error:i32= exi_basetypes_encoder_nbit_uint(stream, 1, 0);
      stream.write_nbit_uint(1,0)?;
       Ok(())
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleList; type={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleListType; base type=SASchedulesType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: SAScheduleTuple, SAScheduleTupleType (1, 3);
  pub fn encode_ISO2SAScheduleListType(stream: &mut ExiBitstream, SAScheduleListType: &ISO2SAScheduleListType )->Result<(), BitstreamError> {
      let mut grammar_id = 203;
      let mut done = false;
      let mut SAScheduleTuple_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           203=>{
               // Grammar: ID=203; read/write bits=1; START (SAScheduleTuple)
              if (SAScheduleTuple_currentIndex < SAScheduleListType.SAScheduleTuple.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (SAScheduleTupleType); next=204
  
  
                      encode_ISO2SAScheduleTupleType(stream, &SAScheduleListType.SAScheduleTuple[SAScheduleTuple_currentIndex])?;
                      SAScheduleTuple_currentIndex+=1;
                      grammar_id = 204;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           204=>{
               // Grammar: ID=204; read/write bits=2; START (SAScheduleTuple), END Element
              if (SAScheduleTuple_currentIndex < SAScheduleListType.SAScheduleTuple.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SAScheduleTupleType); next=205
  
  
                      encode_ISO2SAScheduleTupleType(stream, &SAScheduleListType.SAScheduleTuple[SAScheduleTuple_currentIndex])?;
                      SAScheduleTuple_currentIndex+=1;
                      grammar_id = 205;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           205=>{
               // Grammar: ID=205; read/write bits=2; START (SAScheduleTuple), END Element
              if (SAScheduleTuple_currentIndex < SAScheduleListType.SAScheduleTuple.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (SAScheduleTupleType); next=3
  
  
                      encode_ISO2SAScheduleTupleType(stream, &SAScheduleListType.SAScheduleTuple[SAScheduleTuple_currentIndex])?;
                      SAScheduleTuple_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVChargeParameterType; base type=EVChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DepartureTime, unsignedInt (0, 1); EAmount, PhysicalValueType (1, 1); EVMaxVoltage, PhysicalValueType (1, 1); EVMaxCurrent, PhysicalValueType (1, 1); EVMinCurrent, PhysicalValueType (1, 1);
  pub fn encode_ISO2AC_EVChargeParameterType(stream: &mut ExiBitstream, AC_EVChargeParameterType: &ISO2AC_EVChargeParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 206;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           206=>{
               // Grammar: ID=206; read/write bits=2; START (DepartureTime), START (EAmount)
              if AC_EVChargeParameterType.DepartureTime.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (DepartureTime, unsignedLong); next=207
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_u32(AC_EVChargeParameterType.DepartureTime as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 207;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (EAmount, PhysicalValueType); next=208
  
  
  
                      encode_ISO2PhysicalValueType(stream,&AC_EVChargeParameterType.EAmount)?;
  
                      grammar_id = 208;
              }
  }
           207=>{
               // Grammar: ID=207; read/write bits=1; START (EAmount)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=208
  
  
  
                  encode_ISO2PhysicalValueType(stream,&AC_EVChargeParameterType.EAmount)?;
  
                  grammar_id = 208;
  }
           208=>{
               // Grammar: ID=208; read/write bits=1; START (EVMaxVoltage)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=209
  
  
  
                  encode_ISO2PhysicalValueType(stream,&AC_EVChargeParameterType.EVMaxVoltage)?;
  
                  grammar_id = 209;
  }
           209=>{
               // Grammar: ID=209; read/write bits=1; START (EVMaxCurrent)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=210
  
  
  
                  encode_ISO2PhysicalValueType(stream,&AC_EVChargeParameterType.EVMaxCurrent)?;
  
                  grammar_id = 210;
  }
           210=>{
               // Grammar: ID=210; read/write bits=1; START (EVMinCurrent)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=3
  
  
  
                  encode_ISO2PhysicalValueType(stream,&AC_EVChargeParameterType.EVMinCurrent)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVChargeParameterType; base type=EVChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DepartureTime, unsignedInt (0, 1); DC_EVStatus, DC_EVStatusType (1, 1); EVMaximumCurrentLimit, PhysicalValueType (1, 1); EVMaximumPowerLimit, PhysicalValueType (0, 1); EVMaximumVoltageLimit, PhysicalValueType (1, 1); EVEnergyCapacity, PhysicalValueType (0, 1); EVEnergyRequest, PhysicalValueType (0, 1); FullSOC, percentValueType (0, 1); BulkSOC, percentValueType (0, 1);
  pub fn encode_ISO2DC_EVChargeParameterType(stream: &mut ExiBitstream, DC_EVChargeParameterType: &ISO2DC_EVChargeParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 211;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           211=>{
               // Grammar: ID=211; read/write bits=2; START (DepartureTime), START (DC_EVStatus)
              if DC_EVChargeParameterType.DepartureTime.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (DepartureTime, unsignedLong); next=212
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_u32(DC_EVChargeParameterType.DepartureTime as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 212;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (DC_EVStatus, EVStatusType); next=213
  
  
  
                      encode_ISO2DC_EVStatusType(stream,&DC_EVChargeParameterType.DC_EVStatus)?;
  
                      grammar_id = 213;
              }
  }
           212=>{
               // Grammar: ID=212; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVStatusType); next=213
  
  
  
                  encode_ISO2DC_EVStatusType(stream,&DC_EVChargeParameterType.DC_EVStatus)?;
  
                  grammar_id = 213;
  }
           213=>{
               // Grammar: ID=213; read/write bits=1; START (EVMaximumCurrentLimit)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=214
  
  
  
                  encode_ISO2PhysicalValueType(stream,&DC_EVChargeParameterType.EVMaximumCurrentLimit)?;
  
                  grammar_id = 214;
  }
           214=>{
               // Grammar: ID=214; read/write bits=2; START (EVMaximumPowerLimit), START (EVMaximumVoltageLimit)
              if DC_EVChargeParameterType.EVMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (EVMaximumPowerLimit, PhysicalValueType); next=215
  
  
   
                      encode_ISO2PhysicalValueType(stream,&DC_EVChargeParameterType.EVMaximumPowerLimit.unwrap())?;
  
                      grammar_id = 215;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (EVMaximumVoltageLimit, PhysicalValueType); next=216
  
  
  
                      encode_ISO2PhysicalValueType(stream,&DC_EVChargeParameterType.EVMaximumVoltageLimit)?;
  
                      grammar_id = 216;
              }
  }
           215=>{
               // Grammar: ID=215; read/write bits=1; START (EVMaximumVoltageLimit)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=216
  
  
  
                  encode_ISO2PhysicalValueType(stream,&DC_EVChargeParameterType.EVMaximumVoltageLimit)?;
  
                  grammar_id = 216;
  }
           216=>{
               // Grammar: ID=216; read/write bits=3; START (EVEnergyCapacity), START (EVEnergyRequest), START (FullSOC), START (BulkSOC), END Element
              if DC_EVChargeParameterType.EVEnergyCapacity.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (EVEnergyCapacity, PhysicalValueType); next=217
  
  
   
                      encode_ISO2PhysicalValueType(stream,&DC_EVChargeParameterType.EVEnergyCapacity.unwrap())?;
  
                      grammar_id = 217;
              }
              else if DC_EVChargeParameterType.EVEnergyRequest.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (EVEnergyRequest, PhysicalValueType); next=218
  
  
   
                      encode_ISO2PhysicalValueType(stream,&DC_EVChargeParameterType.EVEnergyRequest.unwrap())?;
  
                      grammar_id = 218;
              }
              else if DC_EVChargeParameterType.FullSOC.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (FullSOC, byte); next=219
  
  
                      stream.write_nbit_uint(1, 0)?; 
  
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.FullSOC.unwrap() as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 219;
  
  
  
              }
              else if DC_EVChargeParameterType.BulkSOC.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (BulkSOC, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?; 
  
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.BulkSOC.unwrap() as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 4 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           217=>{
               // Grammar: ID=217; read/write bits=3; START (EVEnergyRequest), START (FullSOC), START (BulkSOC), END Element
              if DC_EVChargeParameterType.EVEnergyRequest.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (EVEnergyRequest, PhysicalValueType); next=218
  
  
   
                      encode_ISO2PhysicalValueType(stream,&DC_EVChargeParameterType.EVEnergyRequest.unwrap())?;
  
                      grammar_id = 218;
              }
              else if DC_EVChargeParameterType.FullSOC.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (FullSOC, byte); next=219
  
  
                      stream.write_nbit_uint(1, 0)?; 
  
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.FullSOC.unwrap() as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 219;
  
  
  
              }
              else if DC_EVChargeParameterType.BulkSOC.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (BulkSOC, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?; 
  
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.BulkSOC.unwrap() as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           218=>{
               // Grammar: ID=218; read/write bits=2; START (FullSOC), START (BulkSOC), END Element
              if DC_EVChargeParameterType.FullSOC.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (FullSOC, byte); next=219
  
  
                      stream.write_nbit_uint(1, 0)?; 
  
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.FullSOC.unwrap() as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 219;
  
  
  
              }
              else if DC_EVChargeParameterType.BulkSOC.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (BulkSOC, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?; 
  
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.BulkSOC.unwrap() as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           219=>{
               // Grammar: ID=219; read/write bits=2; START (BulkSOC), END Element
              if DC_EVChargeParameterType.BulkSOC.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (BulkSOC, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?; 
  
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.BulkSOC.unwrap() as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVChargeParameterType; base type=; content type=ELEMENT-ONLY;
  //          abstract=True; final=False;
  // Particle: DepartureTime, unsignedInt (0, 1); AC_EVChargeParameter, AC_EVChargeParameterType (1, 1); DC_EVChargeParameter, DC_EVChargeParameterType (1, 1);
  pub fn encode_ISO2EVChargeParameterType(stream: &mut ExiBitstream, EVChargeParameterType: &ISO2EVChargeParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 220;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           220=>{
               // Grammar: ID=220; read/write bits=2; START (DepartureTime), START (AC_EVChargeParameter)
              if EVChargeParameterType.DepartureTime.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (DepartureTime, unsignedLong); next=221
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_u32(EVChargeParameterType.DepartureTime as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 221;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (AC_EVChargeParameter, EVChargeParameterType); next=222
  
  
  
                      encode_ISO2AC_EVChargeParameterType(stream,&EVChargeParameterType.AC_EVChargeParameter)?;
  
                      grammar_id = 222;
              }
  }
           221=>{
               // Grammar: ID=221; read/write bits=1; START (AC_EVChargeParameter)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVChargeParameterType); next=222
  
  
  
                  encode_ISO2AC_EVChargeParameterType(stream,&EVChargeParameterType.AC_EVChargeParameter)?;
  
                  grammar_id = 222;
  }
           222=>{
               // Grammar: ID=222; read/write bits=1; START (DC_EVChargeParameter)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVChargeParameterType); next=3
  
  
  
                  encode_ISO2DC_EVChargeParameterType(stream,&EVChargeParameterType.DC_EVChargeParameter)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingProfile; type={urn:iso:15118:2:2013:MsgDataTypes}ChargingProfileType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ProfileEntry, ProfileEntryType (1, 24);
  pub fn encode_ISO2ChargingProfileType(stream: &mut ExiBitstream, ChargingProfileType: &ISO2ChargingProfileType )->Result<(), BitstreamError> {
      let mut grammar_id = 223;
      let mut done = false;
      let mut ProfileEntry_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           223=>{
               // Grammar: ID=223; read/write bits=1; START (ProfileEntry)
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (ProfileEntryType); next=224
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 224;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           224=>{
               // Grammar: ID=224; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=225
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 225;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           225=>{
               // Grammar: ID=225; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=226
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 226;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           226=>{
               // Grammar: ID=226; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=227
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 227;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           227=>{
               // Grammar: ID=227; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=228
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 228;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           228=>{
               // Grammar: ID=228; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=229
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 229;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           229=>{
               // Grammar: ID=229; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=230
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 230;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           230=>{
               // Grammar: ID=230; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=231
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 231;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           231=>{
               // Grammar: ID=231; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=232
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 232;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           232=>{
               // Grammar: ID=232; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=233
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 233;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           233=>{
               // Grammar: ID=233; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=234
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 234;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           234=>{
               // Grammar: ID=234; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=235
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 235;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           235=>{
               // Grammar: ID=235; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=236
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 236;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           236=>{
               // Grammar: ID=236; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=237
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 237;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           237=>{
               // Grammar: ID=237; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=238
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 238;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           238=>{
               // Grammar: ID=238; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=239
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 239;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           239=>{
               // Grammar: ID=239; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=240
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 240;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           240=>{
               // Grammar: ID=240; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=241
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 241;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           241=>{
               // Grammar: ID=241; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=242
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 242;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           242=>{
               // Grammar: ID=242; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=243
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 243;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           243=>{
               // Grammar: ID=243; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=244
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 244;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           244=>{
               // Grammar: ID=244; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=245
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 245;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           245=>{
               // Grammar: ID=245; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=246
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 246;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           246=>{
               // Grammar: ID=246; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ProfileEntryType); next=3
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceList; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Service, ServiceType (1, 8);
  pub fn encode_ISO2ServiceListType(stream: &mut ExiBitstream, ServiceListType: &ISO2ServiceListType )->Result<(), BitstreamError> {
      let mut grammar_id = 247;
      let mut done = false;
      let mut Service_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           247=>{
               // Grammar: ID=247; read/write bits=1; START (Service)
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(1,0 as u32)?;
                  // Event: START (ServiceType); next=248
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 248;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           248=>{
               // Grammar: ID=248; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ServiceType); next=249
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 249;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           249=>{
               // Grammar: ID=249; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ServiceType); next=250
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 250;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           250=>{
               // Grammar: ID=250; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ServiceType); next=251
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 251;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           251=>{
               // Grammar: ID=251; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ServiceType); next=252
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 252;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           252=>{
               // Grammar: ID=252; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ServiceType); next=253
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 253;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           253=>{
               // Grammar: ID=253; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ServiceType); next=254
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 254;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           254=>{
               // Grammar: ID=254; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0 as u32)?;
                  // Event: START (ServiceType); next=3
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVSEChargeParameterType; base type=; content type=empty;
  //          abstract=True; final=False;
  fn encode_ISO2EVSEChargeParameterType(stream: &mut ExiBitstream,EVSEChargeParameterType: &ISO2EVSEChargeParameterType)->Result<(),BitstreamError>{
      // Element has no particles, so the function just encodes END Element
      let _ = EVSEChargeParameterType;// To silence the unused variable warning
  
  //    error:i32= exi_basetypes_encoder_nbit_uint(stream, 1, 0);
      stream.write_nbit_uint(1,0)?;
       Ok(())
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEChargeParameterType; base type=EVSEChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: AC_EVSEStatus, AC_EVSEStatusType (1, 1); EVSENominalVoltage, PhysicalValueType (1, 1); EVSEMaxCurrent, PhysicalValueType (1, 1);
  pub fn encode_ISO2AC_EVSEChargeParameterType(stream: &mut ExiBitstream, AC_EVSEChargeParameterType: &ISO2AC_EVSEChargeParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 255;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           255=>{
               // Grammar: ID=255; read/write bits=1; START (AC_EVSEStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVSEStatusType); next=256
  
  
  
                  encode_ISO2AC_EVSEStatusType(stream,&AC_EVSEChargeParameterType.AC_EVSEStatus)?;
  
                  grammar_id = 256;
  }
           256=>{
               // Grammar: ID=256; read/write bits=1; START (EVSENominalVoltage)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=257
  
  
  
                  encode_ISO2PhysicalValueType(stream,&AC_EVSEChargeParameterType.EVSENominalVoltage)?;
  
                  grammar_id = 257;
  }
           257=>{
               // Grammar: ID=257; read/write bits=1; START (EVSEMaxCurrent)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=3
  
  
  
                  encode_ISO2PhysicalValueType(stream,&AC_EVSEChargeParameterType.EVSEMaxCurrent)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEChargeParameterType; base type=EVSEChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEMaximumCurrentLimit, PhysicalValueType (1, 1); EVSEMaximumPowerLimit, PhysicalValueType (1, 1); EVSEMaximumVoltageLimit, PhysicalValueType (1, 1); EVSEMinimumCurrentLimit, PhysicalValueType (1, 1); EVSEMinimumVoltageLimit, PhysicalValueType (1, 1); EVSECurrentRegulationTolerance, PhysicalValueType (0, 1); EVSEPeakCurrentRipple, PhysicalValueType (1, 1); EVSEEnergyToBeDelivered, PhysicalValueType (0, 1);
  pub fn encode_ISO2DC_EVSEChargeParameterType(stream: &mut ExiBitstream, DC_EVSEChargeParameterType: &ISO2DC_EVSEChargeParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 258;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           258=>{
               // Grammar: ID=258; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVSEStatusType); next=259
  
  
  
                  encode_ISO2DC_EVSEStatusType(stream,&DC_EVSEChargeParameterType.DC_EVSEStatus)?;
  
                  grammar_id = 259;
  }
           259=>{
               // Grammar: ID=259; read/write bits=1; START (EVSEMaximumCurrentLimit)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=260
  
  
  
                  encode_ISO2PhysicalValueType(stream,&DC_EVSEChargeParameterType.EVSEMaximumCurrentLimit)?;
  
                  grammar_id = 260;
  }
           260=>{
               // Grammar: ID=260; read/write bits=1; START (EVSEMaximumPowerLimit)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=261
  
  
  
                  encode_ISO2PhysicalValueType(stream,&DC_EVSEChargeParameterType.EVSEMaximumPowerLimit)?;
  
                  grammar_id = 261;
  }
           261=>{
               // Grammar: ID=261; read/write bits=1; START (EVSEMaximumVoltageLimit)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=262
  
  
  
                  encode_ISO2PhysicalValueType(stream,&DC_EVSEChargeParameterType.EVSEMaximumVoltageLimit)?;
  
                  grammar_id = 262;
  }
           262=>{
               // Grammar: ID=262; read/write bits=1; START (EVSEMinimumCurrentLimit)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=263
  
  
  
                  encode_ISO2PhysicalValueType(stream,&DC_EVSEChargeParameterType.EVSEMinimumCurrentLimit)?;
  
                  grammar_id = 263;
  }
           263=>{
               // Grammar: ID=263; read/write bits=1; START (EVSEMinimumVoltageLimit)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=264
  
  
  
                  encode_ISO2PhysicalValueType(stream,&DC_EVSEChargeParameterType.EVSEMinimumVoltageLimit)?;
  
                  grammar_id = 264;
  }
           264=>{
               // Grammar: ID=264; read/write bits=2; START (EVSECurrentRegulationTolerance), START (EVSEPeakCurrentRipple)
              if DC_EVSEChargeParameterType.EVSECurrentRegulationTolerance.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (EVSECurrentRegulationTolerance, PhysicalValueType); next=265
  
  
   
                      encode_ISO2PhysicalValueType(stream,&DC_EVSEChargeParameterType.EVSECurrentRegulationTolerance.unwrap())?;
  
                      grammar_id = 265;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (EVSEPeakCurrentRipple, PhysicalValueType); next=266
  
  
  
                      encode_ISO2PhysicalValueType(stream,&DC_EVSEChargeParameterType.EVSEPeakCurrentRipple)?;
  
                      grammar_id = 266;
              }
  }
           265=>{
               // Grammar: ID=265; read/write bits=1; START (EVSEPeakCurrentRipple)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=266
  
  
  
                  encode_ISO2PhysicalValueType(stream,&DC_EVSEChargeParameterType.EVSEPeakCurrentRipple)?;
  
                  grammar_id = 266;
  }
           266=>{
               // Grammar: ID=266; read/write bits=2; START (EVSEEnergyToBeDelivered), END Element
              if DC_EVSEChargeParameterType.EVSEEnergyToBeDelivered.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (EVSEEnergyToBeDelivered, PhysicalValueType); next=3
  
  
   
                      encode_ISO2PhysicalValueType(stream,&DC_EVSEChargeParameterType.EVSEEnergyToBeDelivered.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ContractSignatureEncryptedPrivateKey; type={urn:iso:15118:2:2013:MsgDataTypes}ContractSignatureEncryptedPrivateKeyType; base type=privateKeyType; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); CONTENT, ContractSignatureEncryptedPrivateKeyType (1, 1);
  pub fn encode_ISO2ContractSignatureEncryptedPrivateKeyType(stream: &mut ExiBitstream, ContractSignatureEncryptedPrivateKeyType: &ISO2ContractSignatureEncryptedPrivateKeyType )->Result<(), BitstreamError> {
      let mut grammar_id = 267;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           267=>{
               // Grammar: ID=267; read/write bits=1; START (Id)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (NCName); next=268
  
              // string should not be found in table, so add 2
  
               stream.write_u16(ContractSignatureEncryptedPrivateKeyType.Id.len() as u16)?;
              stream.write_characters(&ContractSignatureEncryptedPrivateKeyType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
  
              grammar_id = 268;
  }
           268=>{
               // Grammar: ID=268; read/write bits=1; START (CONTENT)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=3
                  stream.write_u16(ContractSignatureEncryptedPrivateKeyType.CONTENT.len() as u16)?;
                  stream.write_bytes(&ContractSignatureEncryptedPrivateKeyType.CONTENT)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVPowerDeliveryParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVPowerDeliveryParameterType; base type=; content type=empty;
  //          abstract=True; final=False;
  fn encode_ISO2EVPowerDeliveryParameterType(stream: &mut ExiBitstream,EVPowerDeliveryParameterType: &ISO2EVPowerDeliveryParameterType)->Result<(),BitstreamError>{
      // Element has no particles, so the function just encodes END Element
      let _ = EVPowerDeliveryParameterType;// To silence the unused variable warning
  
  //    error:i32= exi_basetypes_encoder_nbit_uint(stream, 1, 0);
      stream.write_nbit_uint(1,0)?;
       Ok(())
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVPowerDeliveryParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVPowerDeliveryParameterType; base type=EVPowerDeliveryParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1); BulkChargingComplete, boolean (0, 1); ChargingComplete, boolean (1, 1);
  pub fn encode_ISO2DC_EVPowerDeliveryParameterType(stream: &mut ExiBitstream, DC_EVPowerDeliveryParameterType: &ISO2DC_EVPowerDeliveryParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 269;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           269=>{
               // Grammar: ID=269; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVStatusType); next=270
  
  
  
                  encode_ISO2DC_EVStatusType(stream,&DC_EVPowerDeliveryParameterType.DC_EVStatus)?;
  
                  grammar_id = 270;
  }
           270=>{
               // Grammar: ID=270; read/write bits=2; START (BulkChargingComplete), START (ChargingComplete)
              if DC_EVPowerDeliveryParameterType.BulkChargingComplete.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (BulkChargingComplete, boolean); next=271
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( DC_EVPowerDeliveryParameterType.BulkChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 271;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (ChargingComplete, boolean); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( DC_EVPowerDeliveryParameterType.ChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  
              }
  }
           271=>{
               // Grammar: ID=271; read/write bits=1; START (ChargingComplete)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (boolean); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( DC_EVPowerDeliveryParameterType.ChargingComplete)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}DHpublickey; type={urn:iso:15118:2:2013:MsgDataTypes}DiffieHellmanPublickeyType; base type=dHpublickeyType; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); CONTENT, DiffieHellmanPublickeyType (1, 1);
  pub fn encode_ISO2DiffieHellmanPublickeyType(stream: &mut ExiBitstream, DiffieHellmanPublickeyType: &ISO2DiffieHellmanPublickeyType )->Result<(), BitstreamError> {
      let mut grammar_id = 272;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           272=>{
               // Grammar: ID=272; read/write bits=1; START (Id)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (NCName); next=273
  
              // string should not be found in table, so add 2
  
               stream.write_u16(DiffieHellmanPublickeyType.Id.len() as u16)?;
              stream.write_characters(&DiffieHellmanPublickeyType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
  
              grammar_id = 273;
  }
           273=>{
               // Grammar: ID=273; read/write bits=1; START (CONTENT)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=3
                  stream.write_u16(DiffieHellmanPublickeyType.CONTENT.len() as u16)?;
                  stream.write_bytes(&DiffieHellmanPublickeyType.CONTENT)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}eMAID; type={urn:iso:15118:2:2013:MsgDataTypes}EMAIDType; base type=eMAIDType; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); CONTENT, EMAIDType (1, 1);
  pub fn encode_ISO2EMAIDType(stream: &mut ExiBitstream, EMAIDType: &ISO2EMAIDType )->Result<(), BitstreamError> {
      let mut grammar_id = 274;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           274=>{
               // Grammar: ID=274; read/write bits=1; START (Id)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (NCName); next=275
  
              // string should not be found in table, so add 2
  
               stream.write_u16(EMAIDType.Id.len() as u16)?;
              stream.write_characters(&EMAIDType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
  
              grammar_id = 275;
  }
           275=>{
               // Grammar: ID=275; read/write bits=1; START (CONTENT)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=3
  
              // string should not be found in table, so add 2
  
               stream.write_u16(EMAIDType.CONTENT.len() as u16)?;
              stream.write_characters(&EMAIDType.CONTENT.to_string(), ISO2CONTENT_CHARACTER_SIZE)?;
  
              grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeterInfo; type={urn:iso:15118:2:2013:MsgDataTypes}MeterInfoType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: MeterID, meterIDType (1, 1); MeterReading, unsignedLong (0, 1); SigMeterReading, sigMeterReadingType (0, 1); MeterStatus, meterStatusType (0, 1); TMeter, long (0, 1);
  pub fn encode_ISO2MeterInfoType(stream: &mut ExiBitstream, MeterInfoType: &ISO2MeterInfoType )->Result<(), BitstreamError> {
      let mut grammar_id = 276;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           276=>{
               // Grammar: ID=276; read/write bits=1; START (MeterID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=277
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
  
                   stream.write_u16(MeterInfoType.MeterID.len() as u16)?;
                  stream.write_characters(&MeterInfoType.MeterID.to_string(), ISO2MeterID_CHARACTER_SIZE)?;
  
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 277;
  }
           277=>{
               // Grammar: ID=277; read/write bits=3; START (MeterReading), START (SigMeterReading), START (MeterStatus), START (TMeter), END Element
              if MeterInfoType.MeterReading.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (MeterReading, nonNegativeInteger); next=278
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_u64(MeterInfoType.MeterReading as u64)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 278;
  
  
  
              }
              else if MeterInfoType.SigMeterReading.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (SigMeterReading, base64Binary); next=279
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(MeterInfoType.SigMeterReading.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &MeterInfoType.SigMeterReading.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 279;
  
  
  
              }
              else if MeterInfoType.MeterStatus.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (MeterStatus, short); next=280
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( MeterInfoType.MeterStatus as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 280;
  
  
  
  
              }
              else if MeterInfoType.TMeter.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (TMeter, integer); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i64(MeterInfoType.TMeter)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 4 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           278=>{
               // Grammar: ID=278; read/write bits=3; START (SigMeterReading), START (MeterStatus), START (TMeter), END Element
              if MeterInfoType.SigMeterReading.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (SigMeterReading, base64Binary); next=279
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(MeterInfoType.SigMeterReading.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &MeterInfoType.SigMeterReading.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 279;
  
  
  
              }
              else if MeterInfoType.MeterStatus.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (MeterStatus, short); next=280
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( MeterInfoType.MeterStatus as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 280;
  
  
  
  
              }
              else if MeterInfoType.TMeter.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (TMeter, integer); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i64(MeterInfoType.TMeter)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           279=>{
               // Grammar: ID=279; read/write bits=2; START (MeterStatus), START (TMeter), END Element
              if MeterInfoType.MeterStatus.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (MeterStatus, short); next=280
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( MeterInfoType.MeterStatus as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 280;
  
  
  
  
              }
              else if MeterInfoType.TMeter.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (TMeter, integer); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i64(MeterInfoType.TMeter)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           280=>{
               // Grammar: ID=280; read/write bits=2; START (TMeter), END Element
              if MeterInfoType.TMeter.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (TMeter, integer); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i64(MeterInfoType.TMeter)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}Header; type={urn:iso:15118:2:2013:MsgHeader}MessageHeaderType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SessionID, sessionIDType (1, 1); Notification, NotificationType (0, 1); Signature, SignatureType (0, 1);
  pub fn encode_ISO2MessageHeaderType(stream: &mut ExiBitstream, MessageHeaderType: &ISO2MessageHeaderType )->Result<(), BitstreamError> {
      let mut grammar_id = 281;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           281=>{
               // Grammar: ID=281; read/write bits=1; START (SessionID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (hexBinary); next=282
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16( MessageHeaderType.SessionID.len() as u16)?;
                  stream.write_bytes( &MessageHeaderType.SessionID)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 282;
  
  
  
  }
           282=>{
               // Grammar: ID=282; read/write bits=2; START (Notification), START (Signature), END Element
              if MessageHeaderType.Notification.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Notification, NotificationType); next=283
  
  
   
                      encode_ISO2NotificationType(stream,&MessageHeaderType.Notification.unwrap())?;
  
                      grammar_id = 283;
              }
              else if MessageHeaderType.Signature.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (Signature, SignatureType); next=3
  
  
   
                      encode_ISO2SignatureType(stream,&MessageHeaderType.Signature.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           283=>{
               // Grammar: ID=283; read/write bits=2; START (Signature), END Element
              if MessageHeaderType.Signature.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Signature, SignatureType); next=3
  
  
   
                      encode_ISO2SignatureType(stream,&MessageHeaderType.Signature.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryRes; type={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); PaymentOptionList, PaymentOptionListType (1, 1); ChargeService, ChargeServiceType (1, 1); ServiceList, ServiceListType (0, 1);
  pub fn encode_ISO2ServiceDiscoveryResType(stream: &mut ExiBitstream, ServiceDiscoveryResType: &ISO2ServiceDiscoveryResType )->Result<(), BitstreamError> {
      let mut grammar_id = 284;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           284=>{
               // Grammar: ID=284; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=285
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, ServiceDiscoveryResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 285;
  }
           285=>{
               // Grammar: ID=285; read/write bits=1; START (PaymentOptionList)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PaymentOptionListType); next=286
  
  
  
                  encode_ISO2PaymentOptionListType(stream,&ServiceDiscoveryResType.PaymentOptionList)?;
  
                  grammar_id = 286;
  }
           286=>{
               // Grammar: ID=286; read/write bits=1; START (ChargeService)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (ServiceType); next=287
  
  
  
                  encode_ISO2ChargeServiceType(stream,&ServiceDiscoveryResType.ChargeService)?;
  
                  grammar_id = 287;
  }
           287=>{
               // Grammar: ID=287; read/write bits=2; START (ServiceList), END Element
              if ServiceDiscoveryResType.ServiceList.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ServiceList, ServiceListType); next=3
  
  
   
                      encode_ISO2ServiceListType(stream,&ServiceDiscoveryResType.ServiceList.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionReq; type={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: SelectedPaymentOption, paymentOptionType (1, 1); SelectedServiceList, SelectedServiceListType (1, 1);
  pub fn encode_ISO2PaymentServiceSelectionReqType(stream: &mut ExiBitstream, PaymentServiceSelectionReqType: &ISO2PaymentServiceSelectionReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 288;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           288=>{
               // Grammar: ID=288; read/write bits=1; START (SelectedPaymentOption)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=289
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(1, PaymentServiceSelectionReqType.SelectedPaymentOption as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 289;
  }
           289=>{
               // Grammar: ID=289; read/write bits=1; START (SelectedServiceList)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (SelectedServiceListType); next=3
  
  
  
                  encode_ISO2SelectedServiceListType(stream,&PaymentServiceSelectionReqType.SelectedServiceList)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}WeldingDetectionReq; type={urn:iso:15118:2:2013:MsgBody}WeldingDetectionReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1);
  pub fn encode_ISO2WeldingDetectionReqType(stream: &mut ExiBitstream, WeldingDetectionReqType: &ISO2WeldingDetectionReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 290;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           290=>{
               // Grammar: ID=290; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVStatusType); next=3
  
  
  
                  encode_ISO2DC_EVStatusType(stream,&WeldingDetectionReqType.DC_EVStatus)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}WeldingDetectionRes; type={urn:iso:15118:2:2013:MsgBody}WeldingDetectionResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1);
  pub fn encode_ISO2WeldingDetectionResType(stream: &mut ExiBitstream, WeldingDetectionResType: &ISO2WeldingDetectionResType )->Result<(), BitstreamError> {
      let mut grammar_id = 291;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           291=>{
               // Grammar: ID=291; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=292
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, WeldingDetectionResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 292;
  }
           292=>{
               // Grammar: ID=292; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVSEStatusType); next=293
  
  
  
                  encode_ISO2DC_EVSEStatusType(stream,&WeldingDetectionResType.DC_EVSEStatus)?;
  
                  grammar_id = 293;
  }
           293=>{
               // Grammar: ID=293; read/write bits=1; START (EVSEPresentVoltage)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=3
  
  
  
                  encode_ISO2PhysicalValueType(stream,&WeldingDetectionResType.EVSEPresentVoltage)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateUpdateReq; type={urn:iso:15118:2:2013:MsgBody}CertificateUpdateReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); eMAID, eMAIDType (1, 1); ListOfRootCertificateIDs, ListOfRootCertificateIDsType (1, 1);
  pub fn encode_ISO2CertificateUpdateReqType(stream: &mut ExiBitstream, CertificateUpdateReqType: &ISO2CertificateUpdateReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 294;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           294=>{
               // Grammar: ID=294; read/write bits=1; START (Id)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (NCName); next=295
  
              // string should not be found in table, so add 2
  
               stream.write_u16(CertificateUpdateReqType.Id.len() as u16)?;
              stream.write_characters(&CertificateUpdateReqType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
  
              grammar_id = 295;
  }
           295=>{
               // Grammar: ID=295; read/write bits=1; START (ContractSignatureCertChain)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (CertificateChainType); next=296
  
  
  
                  encode_ISO2CertificateChainType(stream,&CertificateUpdateReqType.ContractSignatureCertChain)?;
  
                  grammar_id = 296;
  }
           296=>{
               // Grammar: ID=296; read/write bits=1; START (eMAID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=297
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
  
                   stream.write_u16(CertificateUpdateReqType.eMAID.len() as u16)?;
                  stream.write_characters(&CertificateUpdateReqType.eMAID.to_string(), ISO2eMAID_CHARACTER_SIZE)?;
  
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 297;
  }
           297=>{
               // Grammar: ID=297; read/write bits=1; START (ListOfRootCertificateIDs)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (ListOfRootCertificateIDsType); next=3
  
  
  
                  encode_ISO2ListOfRootCertificateIDsType(stream,&CertificateUpdateReqType.ListOfRootCertificateIDs)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PreChargeRes; type={urn:iso:15118:2:2013:MsgBody}PreChargeResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1);
  pub fn encode_ISO2PreChargeResType(stream: &mut ExiBitstream, PreChargeResType: &ISO2PreChargeResType )->Result<(), BitstreamError> {
      let mut grammar_id = 298;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           298=>{
               // Grammar: ID=298; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=299
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, PreChargeResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 299;
  }
           299=>{
               // Grammar: ID=299; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVSEStatusType); next=300
  
  
  
                  encode_ISO2DC_EVSEStatusType(stream,&PreChargeResType.DC_EVSEStatus)?;
  
                  grammar_id = 300;
  }
           300=>{
               // Grammar: ID=300; read/write bits=1; START (EVSEPresentVoltage)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=3
  
  
  
                  encode_ISO2PhysicalValueType(stream,&PreChargeResType.EVSEPresentVoltage)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDetailRes; type={urn:iso:15118:2:2013:MsgBody}ServiceDetailResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); ServiceID, serviceIDType (1, 1); ServiceParameterList, ServiceParameterListType (0, 1);
  pub fn encode_ISO2ServiceDetailResType(stream: &mut ExiBitstream, ServiceDetailResType: &ISO2ServiceDetailResType )->Result<(), BitstreamError> {
      let mut grammar_id = 301;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           301=>{
               // Grammar: ID=301; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=302
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, ServiceDetailResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 302;
  }
           302=>{
               // Grammar: ID=302; read/write bits=1; START (ServiceID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedShort); next=303
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(ServiceDetailResType.ServiceID as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 303;
  
  
  }
           303=>{
               // Grammar: ID=303; read/write bits=2; START (ServiceParameterList), END Element
              if ServiceDetailResType.ServiceParameterList.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ServiceParameterList, ServiceParameterListType); next=3
  
  
   
                      encode_ISO2ServiceParameterListType(stream,&ServiceDetailResType.ServiceParameterList.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentDetailsReq; type={urn:iso:15118:2:2013:MsgBody}PaymentDetailsReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: eMAID, eMAIDType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1);
  pub fn encode_ISO2PaymentDetailsReqType(stream: &mut ExiBitstream, PaymentDetailsReqType: &ISO2PaymentDetailsReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 304;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           304=>{
               // Grammar: ID=304; read/write bits=1; START (eMAID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=305
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
  
                   stream.write_u16(PaymentDetailsReqType.eMAID.len() as u16)?;
                  stream.write_characters(&PaymentDetailsReqType.eMAID.to_string(), ISO2eMAID_CHARACTER_SIZE)?;
  
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 305;
  }
           305=>{
               // Grammar: ID=305; read/write bits=1; START (ContractSignatureCertChain)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (CertificateChainType); next=3
  
  
  
                  encode_ISO2CertificateChainType(stream,&PaymentDetailsReqType.ContractSignatureCertChain)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDetailReq; type={urn:iso:15118:2:2013:MsgBody}ServiceDetailReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ServiceID, serviceIDType (1, 1);
  pub fn encode_ISO2ServiceDetailReqType(stream: &mut ExiBitstream, ServiceDetailReqType: &ISO2ServiceDetailReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 306;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           306=>{
               // Grammar: ID=306; read/write bits=1; START (ServiceID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedShort); next=3
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(ServiceDetailReqType.ServiceID as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentDetailsRes; type={urn:iso:15118:2:2013:MsgBody}PaymentDetailsResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); GenChallenge, genChallengeType (1, 1); EVSETimeStamp, long (1, 1);
  pub fn encode_ISO2PaymentDetailsResType(stream: &mut ExiBitstream, PaymentDetailsResType: &ISO2PaymentDetailsResType )->Result<(), BitstreamError> {
      let mut grammar_id = 307;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           307=>{
               // Grammar: ID=307; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=308
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, PaymentDetailsResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 308;
  }
           308=>{
               // Grammar: ID=308; read/write bits=1; START (GenChallenge)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=309
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(PaymentDetailsResType.GenChallenge.len() as u16)?;
                          stream.write_bytes( &PaymentDetailsResType.GenChallenge)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 309;
  
  
  }
           309=>{
               // Grammar: ID=309; read/write bits=1; START (EVSETimeStamp)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (integer); next=3
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_i64(PaymentDetailsResType.EVSETimeStamp)?;
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionSetupRes; type={urn:iso:15118:2:2013:MsgBody}SessionSetupResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEID, evseIDType (1, 1); EVSETimeStamp, long (0, 1);
  pub fn encode_ISO2SessionSetupResType(stream: &mut ExiBitstream, SessionSetupResType: &ISO2SessionSetupResType )->Result<(), BitstreamError> {
      let mut grammar_id = 310;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           310=>{
               // Grammar: ID=310; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=311
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, SessionSetupResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 311;
  }
           311=>{
               // Grammar: ID=311; read/write bits=1; START (EVSEID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=312
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
  
                   stream.write_u16(SessionSetupResType.EVSEID.len() as u16)?;
                  stream.write_characters(&SessionSetupResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
  
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 312;
  }
           312=>{
               // Grammar: ID=312; read/write bits=2; START (EVSETimeStamp), END Element
              if SessionSetupResType.EVSETimeStamp.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (EVSETimeStamp, integer); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i64(SessionSetupResType.EVSETimeStamp)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionSetupReq; type={urn:iso:15118:2:2013:MsgBody}SessionSetupReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: EVCCID, evccIDType (1, 1);
  pub fn encode_ISO2SessionSetupReqType(stream: &mut ExiBitstream, SessionSetupReqType: &ISO2SessionSetupReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 313;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           313=>{
               // Grammar: ID=313; read/write bits=1; START (EVCCID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (hexBinary); next=3
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16( SessionSetupReqType.EVCCID.len() as u16)?;
                  stream.write_bytes( &SessionSetupReqType.EVCCID)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionRes; type={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1);
  pub fn encode_ISO2PaymentServiceSelectionResType(stream: &mut ExiBitstream, PaymentServiceSelectionResType: &ISO2PaymentServiceSelectionResType )->Result<(), BitstreamError> {
      let mut grammar_id = 314;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           314=>{
               // Grammar: ID=314; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, PaymentServiceSelectionResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}AuthorizationReq; type={urn:iso:15118:2:2013:MsgBody}AuthorizationReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (0, 1); GenChallenge, genChallengeType (0, 1);
  pub fn encode_ISO2AuthorizationReqType(stream: &mut ExiBitstream, AuthorizationReqType: &ISO2AuthorizationReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 315;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           315=>{
               // Grammar: ID=315; read/write bits=2; START (Id), START (GenChallenge), END Element
              if AuthorizationReqType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Id, NCName); next=316
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(AuthorizationReqType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&AuthorizationReqType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 316;
  
              }
              else if AuthorizationReqType.GenChallenge.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (GenChallenge, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(AuthorizationReqType.GenChallenge.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &AuthorizationReqType.GenChallenge.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           316=>{
               // Grammar: ID=316; read/write bits=2; START (GenChallenge), END Element
              if AuthorizationReqType.GenChallenge.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (GenChallenge, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(AuthorizationReqType.GenChallenge.expect("Value not Initialized").len() as u16)?;
                              stream.write_bytes( &AuthorizationReqType.GenChallenge.expect("Value not Initialized"))?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingStatusReq; type={urn:iso:15118:2:2013:MsgBody}ChargingStatusReqType; base type=BodyBaseType; content type=empty;
  //          abstract=False; final=False; derivation=extension;
  fn encode_ISO2ChargingStatusReqType(stream: &mut ExiBitstream,ChargingStatusReqType: &ISO2ChargingStatusReqType)->Result<(),BitstreamError>{
      // Element has no particles, so the function just encodes END Element
      let _ = ChargingStatusReqType;// To silence the unused variable warning
  
  //    error:i32= exi_basetypes_encoder_nbit_uint(stream, 1, 0);
      stream.write_nbit_uint(1,0)?;
       Ok(())
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}AuthorizationRes; type={urn:iso:15118:2:2013:MsgBody}AuthorizationResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1);
  pub fn encode_ISO2AuthorizationResType(stream: &mut ExiBitstream, AuthorizationResType: &ISO2AuthorizationResType )->Result<(), BitstreamError> {
      let mut grammar_id = 317;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           317=>{
               // Grammar: ID=317; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=318
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, AuthorizationResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 318;
  }
           318=>{
               // Grammar: ID=318; read/write bits=1; START (EVSEProcessing)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, AuthorizationResType.EVSEProcessing as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryRes; type={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1); SAScheduleList, SAScheduleListType (0, 1); SASchedules, SASchedulesType (0, 1); AC_EVSEChargeParameter, AC_EVSEChargeParameterType (0, 1); DC_EVSEChargeParameter, DC_EVSEChargeParameterType (0, 1); EVSEChargeParameter, EVSEChargeParameterType (0, 1);
  pub fn encode_ISO2ChargeParameterDiscoveryResType(stream: &mut ExiBitstream, ChargeParameterDiscoveryResType: &ISO2ChargeParameterDiscoveryResType )->Result<(), BitstreamError> {
      let mut grammar_id = 319;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           319=>{
               // Grammar: ID=319; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=320
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, ChargeParameterDiscoveryResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 320;
  }
           320=>{
               // Grammar: ID=320; read/write bits=1; START (EVSEProcessing)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=321
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, ChargeParameterDiscoveryResType.EVSEProcessing as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 321;
  }
           321=>{
               // Grammar: ID=321; read/write bits=3; START (SAScheduleList), START (SASchedules), START (AC_EVSEChargeParameter), START (DC_EVSEChargeParameter), START (EVSEChargeParameter)
              if ChargeParameterDiscoveryResType.SAScheduleList.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (SAScheduleList, SASchedulesType); next=322
  
  
   
                      encode_ISO2SAScheduleListType(stream,&ChargeParameterDiscoveryResType.SAScheduleList.unwrap())?;
  
                      grammar_id = 322;
              }
              else if ChargeParameterDiscoveryResType.SASchedules.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Abstract element or type: START (SASchedulesType); next=322
  
  
   
                      encode_ISO2SASchedulesType(stream,&ChargeParameterDiscoveryResType.SASchedules.unwrap())?;
  
                      grammar_id = 322;
              }
              else if ChargeParameterDiscoveryResType.AC_EVSEChargeParameter.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (AC_EVSEChargeParameter, EVSEChargeParameterType); next=3
  
  
   
                      encode_ISO2AC_EVSEChargeParameterType(stream,&ChargeParameterDiscoveryResType.AC_EVSEChargeParameter.unwrap())?;
  
                      grammar_id = 3;
              }
              else if ChargeParameterDiscoveryResType.DC_EVSEChargeParameter.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (DC_EVSEChargeParameter, EVSEChargeParameterType); next=3
  
  
   
                      encode_ISO2DC_EVSEChargeParameterType(stream,&ChargeParameterDiscoveryResType.DC_EVSEChargeParameter.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(3, 4 as u32)?;
                  // Abstract element or type: START (EVSEChargeParameterType); next=3
  
  
  
                      encode_ISO2EVSEChargeParameterType(stream,&ChargeParameterDiscoveryResType.EVSEChargeParameter)?;
  
                      grammar_id = 3;
              }
  }
           322=>{
               // Grammar: ID=322; read/write bits=2; START (AC_EVSEChargeParameter), START (DC_EVSEChargeParameter), START (EVSEChargeParameter)
              if ChargeParameterDiscoveryResType.AC_EVSEChargeParameter.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (AC_EVSEChargeParameter, EVSEChargeParameterType); next=3
  
  
   
                      encode_ISO2AC_EVSEChargeParameterType(stream,&ChargeParameterDiscoveryResType.AC_EVSEChargeParameter.unwrap())?;
  
                      grammar_id = 3;
              }
              else if ChargeParameterDiscoveryResType.DC_EVSEChargeParameter.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (DC_EVSEChargeParameter, EVSEChargeParameterType); next=3
  
  
   
                      encode_ISO2DC_EVSEChargeParameterType(stream,&ChargeParameterDiscoveryResType.DC_EVSEChargeParameter.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Abstract element or type: START (EVSEChargeParameterType); next=3
  
  
  
                      encode_ISO2EVSEChargeParameterType(stream,&ChargeParameterDiscoveryResType.EVSEChargeParameter)?;
  
                      grammar_id = 3;
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateUpdateRes; type={urn:iso:15118:2:2013:MsgBody}CertificateUpdateResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); SAProvisioningCertificateChain, CertificateChainType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (1, 1); DHpublickey, DiffieHellmanPublickeyType (1, 1); eMAID, EMAIDType (1, 1); RetryCounter, short (0, 1);
  pub fn encode_ISO2CertificateUpdateResType(stream: &mut ExiBitstream, CertificateUpdateResType: &ISO2CertificateUpdateResType )->Result<(), BitstreamError> {
      let mut grammar_id = 323;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           323=>{
               // Grammar: ID=323; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=324
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, CertificateUpdateResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 324;
  }
           324=>{
               // Grammar: ID=324; read/write bits=1; START (SAProvisioningCertificateChain)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (CertificateChainType); next=325
  
  
  
                  encode_ISO2CertificateChainType(stream,&CertificateUpdateResType.SAProvisioningCertificateChain)?;
  
                  grammar_id = 325;
  }
           325=>{
               // Grammar: ID=325; read/write bits=1; START (ContractSignatureCertChain)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (CertificateChainType); next=326
  
  
  
                  encode_ISO2CertificateChainType(stream,&CertificateUpdateResType.ContractSignatureCertChain)?;
  
                  grammar_id = 326;
  }
           326=>{
               // Grammar: ID=326; read/write bits=1; START (ContractSignatureEncryptedPrivateKey)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (privateKeyType); next=327
  
  
  
                  encode_ISO2ContractSignatureEncryptedPrivateKeyType(stream,&CertificateUpdateResType.ContractSignatureEncryptedPrivateKey)?;
  
                  grammar_id = 327;
  }
           327=>{
               // Grammar: ID=327; read/write bits=1; START (DHpublickey)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (dHpublickeyType); next=328
  
  
  
                  encode_ISO2DiffieHellmanPublickeyType(stream,&CertificateUpdateResType.DHpublickey)?;
  
                  grammar_id = 328;
  }
           328=>{
               // Grammar: ID=328; read/write bits=1; START (eMAID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (eMAIDType); next=329
  
  
  
                  encode_ISO2EMAIDType(stream,&CertificateUpdateResType.eMAID)?;
  
                  grammar_id = 329;
  }
           329=>{
               // Grammar: ID=329; read/write bits=2; START (RetryCounter), END Element
              if CertificateUpdateResType.RetryCounter.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (RetryCounter, int); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( CertificateUpdateResType.RetryCounter as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CurrentDemandRes; type={urn:iso:15118:2:2013:MsgBody}CurrentDemandResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1); EVSEPresentCurrent, PhysicalValueType (1, 1); EVSECurrentLimitAchieved, boolean (1, 1); EVSEVoltageLimitAchieved, boolean (1, 1); EVSEPowerLimitAchieved, boolean (1, 1); EVSEMaximumVoltageLimit, PhysicalValueType (0, 1); EVSEMaximumCurrentLimit, PhysicalValueType (0, 1); EVSEMaximumPowerLimit, PhysicalValueType (0, 1); EVSEID, evseIDType (1, 1); SAScheduleTupleID, SAIDType (1, 1); MeterInfo, MeterInfoType (0, 1); ReceiptRequired, boolean (0, 1);
  pub fn encode_ISO2CurrentDemandResType(stream: &mut ExiBitstream, CurrentDemandResType: &ISO2CurrentDemandResType )->Result<(), BitstreamError> {
      let mut grammar_id = 330;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           330=>{
               // Grammar: ID=330; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=331
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, CurrentDemandResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 331;
  }
           331=>{
               // Grammar: ID=331; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVSEStatusType); next=332
  
  
  
                  encode_ISO2DC_EVSEStatusType(stream,&CurrentDemandResType.DC_EVSEStatus)?;
  
                  grammar_id = 332;
  }
           332=>{
               // Grammar: ID=332; read/write bits=1; START (EVSEPresentVoltage)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=333
  
  
  
                  encode_ISO2PhysicalValueType(stream,&CurrentDemandResType.EVSEPresentVoltage)?;
  
                  grammar_id = 333;
  }
           333=>{
               // Grammar: ID=333; read/write bits=1; START (EVSEPresentCurrent)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=334
  
  
  
                  encode_ISO2PhysicalValueType(stream,&CurrentDemandResType.EVSEPresentCurrent)?;
  
                  grammar_id = 334;
  }
           334=>{
               // Grammar: ID=334; read/write bits=1; START (EVSECurrentLimitAchieved)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (boolean); next=335
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( CurrentDemandResType.EVSECurrentLimitAchieved)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 335;
  
  
  }
           335=>{
               // Grammar: ID=335; read/write bits=1; START (EVSEVoltageLimitAchieved)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (boolean); next=336
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( CurrentDemandResType.EVSEVoltageLimitAchieved)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 336;
  
  
  }
           336=>{
               // Grammar: ID=336; read/write bits=1; START (EVSEPowerLimitAchieved)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (boolean); next=337
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( CurrentDemandResType.EVSEPowerLimitAchieved)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 337;
  
  
  }
           337=>{
               // Grammar: ID=337; read/write bits=3; START (EVSEMaximumVoltageLimit), START (EVSEMaximumCurrentLimit), START (EVSEMaximumPowerLimit), START (EVSEID)
              if CurrentDemandResType.EVSEMaximumVoltageLimit.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (EVSEMaximumVoltageLimit, PhysicalValueType); next=338
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandResType.EVSEMaximumVoltageLimit.unwrap())?;
  
                      grammar_id = 338;
              }
              else if CurrentDemandResType.EVSEMaximumCurrentLimit.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (EVSEMaximumCurrentLimit, PhysicalValueType); next=339
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandResType.EVSEMaximumCurrentLimit.unwrap())?;
  
                      grammar_id = 339;
              }
              else if CurrentDemandResType.EVSEMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (EVSEMaximumPowerLimit, PhysicalValueType); next=340
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandResType.EVSEMaximumPowerLimit.unwrap())?;
  
                      grammar_id = 340;
              }
              else
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (EVSEID, string); next=341
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(CurrentDemandResType.EVSEID.len() as u16)?;
                      stream.write_characters(&CurrentDemandResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 341;
              }
  }
           338=>{
               // Grammar: ID=338; read/write bits=2; START (EVSEMaximumCurrentLimit), START (EVSEMaximumPowerLimit), START (EVSEID)
              if CurrentDemandResType.EVSEMaximumCurrentLimit.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (EVSEMaximumCurrentLimit, PhysicalValueType); next=339
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandResType.EVSEMaximumCurrentLimit.unwrap())?;
  
                      grammar_id = 339;
              }
              else if CurrentDemandResType.EVSEMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (EVSEMaximumPowerLimit, PhysicalValueType); next=340
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandResType.EVSEMaximumPowerLimit.unwrap())?;
  
                      grammar_id = 340;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (EVSEID, string); next=341
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(CurrentDemandResType.EVSEID.len() as u16)?;
                      stream.write_characters(&CurrentDemandResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 341;
              }
  }
           339=>{
               // Grammar: ID=339; read/write bits=2; START (EVSEMaximumPowerLimit), START (EVSEID)
              if CurrentDemandResType.EVSEMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (EVSEMaximumPowerLimit, PhysicalValueType); next=340
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandResType.EVSEMaximumPowerLimit.unwrap())?;
  
                      grammar_id = 340;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (EVSEID, string); next=341
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(CurrentDemandResType.EVSEID.len() as u16)?;
                      stream.write_characters(&CurrentDemandResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 341;
              }
  }
           340=>{
               // Grammar: ID=340; read/write bits=1; START (EVSEID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=341
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
  
                   stream.write_u16(CurrentDemandResType.EVSEID.len() as u16)?;
                  stream.write_characters(&CurrentDemandResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
  
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 341;
  }
           341=>{
               // Grammar: ID=341; read/write bits=1; START (SAScheduleTupleID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedByte); next=342
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(8, CurrentDemandResType.SAScheduleTupleID as u32 - 1)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
  
                  grammar_id = 342;
  
  
  }
           342=>{
               // Grammar: ID=342; read/write bits=2; START (MeterInfo), START (ReceiptRequired), END Element
              if CurrentDemandResType.MeterInfo.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (MeterInfo, MeterInfoType); next=343
  
  
   
                      encode_ISO2MeterInfoType(stream,&CurrentDemandResType.MeterInfo.unwrap())?;
  
                      grammar_id = 343;
              }
              else if CurrentDemandResType.ReceiptRequired.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (ReceiptRequired, boolean); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandResType.ReceiptRequired)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           343=>{
               // Grammar: ID=343; read/write bits=2; START (ReceiptRequired), END Element
              if CurrentDemandResType.ReceiptRequired.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ReceiptRequired, boolean); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandResType.ReceiptRequired)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryReq; type={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: MaxEntriesSAScheduleTuple, unsignedShort (0, 1); RequestedEnergyTransferMode, EnergyTransferModeType (1, 1); AC_EVChargeParameter, AC_EVChargeParameterType (0, 1); DC_EVChargeParameter, DC_EVChargeParameterType (0, 1); EVChargeParameter, EVChargeParameterType (0, 1);
  pub fn encode_ISO2ChargeParameterDiscoveryReqType(stream: &mut ExiBitstream, ChargeParameterDiscoveryReqType: &ISO2ChargeParameterDiscoveryReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 344;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           344=>{
               // Grammar: ID=344; read/write bits=2; START (MaxEntriesSAScheduleTuple), START (RequestedEnergyTransferMode)
              if ChargeParameterDiscoveryReqType.MaxEntriesSAScheduleTuple.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (MaxEntriesSAScheduleTuple, unsignedInt); next=345
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(ChargeParameterDiscoveryReqType.MaxEntriesSAScheduleTuple as u16)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 345;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (RequestedEnergyTransferMode, string); next=346
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(3, ChargeParameterDiscoveryReqType.RequestedEnergyTransferMode as u32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 346;
              }
  }
           345=>{
               // Grammar: ID=345; read/write bits=1; START (RequestedEnergyTransferMode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=346
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(3, ChargeParameterDiscoveryReqType.RequestedEnergyTransferMode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 346;
  }
           346=>{
               // Grammar: ID=346; read/write bits=2; START (AC_EVChargeParameter), START (DC_EVChargeParameter), START (EVChargeParameter)
              if ChargeParameterDiscoveryReqType.AC_EVChargeParameter.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (AC_EVChargeParameter, EVChargeParameterType); next=3
  
  
   
                      encode_ISO2AC_EVChargeParameterType(stream,&ChargeParameterDiscoveryReqType.AC_EVChargeParameter.unwrap())?;
  
                      grammar_id = 3;
              }
              else if ChargeParameterDiscoveryReqType.DC_EVChargeParameter.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (DC_EVChargeParameter, EVChargeParameterType); next=3
  
  
   
                      encode_ISO2DC_EVChargeParameterType(stream,&ChargeParameterDiscoveryReqType.DC_EVChargeParameter.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (EVChargeParameter, EVChargeParameterType); next=3
  
  
  
                      encode_ISO2EVChargeParameterType(stream,&ChargeParameterDiscoveryReqType.EVChargeParameter)?;
  
                      grammar_id = 3;
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CableCheckReq; type={urn:iso:15118:2:2013:MsgBody}CableCheckReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1);
  pub fn encode_ISO2CableCheckReqType(stream: &mut ExiBitstream, CableCheckReqType: &ISO2CableCheckReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 347;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           347=>{
               // Grammar: ID=347; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVStatusType); next=3
  
  
  
                  encode_ISO2DC_EVStatusType(stream,&CableCheckReqType.DC_EVStatus)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PowerDeliveryReq; type={urn:iso:15118:2:2013:MsgBody}PowerDeliveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ChargeProgress, chargeProgressType (1, 1); SAScheduleTupleID, SAIDType (1, 1); ChargingProfile, ChargingProfileType (0, 1); DC_EVPowerDeliveryParameter, DC_EVPowerDeliveryParameterType (0, 1); EVPowerDeliveryParameter, EVPowerDeliveryParameterType (0, 1);
  pub fn encode_ISO2PowerDeliveryReqType(stream: &mut ExiBitstream, PowerDeliveryReqType: &ISO2PowerDeliveryReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 348;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           348=>{
               // Grammar: ID=348; read/write bits=1; START (ChargeProgress)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=349
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, PowerDeliveryReqType.ChargeProgress as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 349;
  }
           349=>{
               // Grammar: ID=349; read/write bits=1; START (SAScheduleTupleID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedByte); next=350
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(8, PowerDeliveryReqType.SAScheduleTupleID as u32 - 1)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
  
                  grammar_id = 350;
  
  
  }
           350=>{
               // Grammar: ID=350; read/write bits=3; START (ChargingProfile), START (DC_EVPowerDeliveryParameter), START (EVPowerDeliveryParameter), END Element
              if PowerDeliveryReqType.ChargingProfile.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (ChargingProfile, ChargingProfileType); next=351
  
  
   
                      encode_ISO2ChargingProfileType(stream,&PowerDeliveryReqType.ChargingProfile.unwrap())?;
  
                      grammar_id = 351;
              }
              else if PowerDeliveryReqType.DC_EVPowerDeliveryParameter.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (DC_EVPowerDeliveryParameter, EVPowerDeliveryParameterType); next=3
  
  
   
                      encode_ISO2DC_EVPowerDeliveryParameterType(stream,&PowerDeliveryReqType.DC_EVPowerDeliveryParameter.unwrap())?;
  
                      grammar_id = 3;
              }
              else if PowerDeliveryReqType.EVPowerDeliveryParameter.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Abstract element or type: START (EVPowerDeliveryParameterType); next=3
  
  
   
                      encode_ISO2EVPowerDeliveryParameterType(stream,&PowerDeliveryReqType.EVPowerDeliveryParameter.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           351=>{
               // Grammar: ID=351; read/write bits=2; START (DC_EVPowerDeliveryParameter), START (EVPowerDeliveryParameter), END Element
              if PowerDeliveryReqType.DC_EVPowerDeliveryParameter.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (DC_EVPowerDeliveryParameter, EVPowerDeliveryParameterType); next=3
  
  
   
                      encode_ISO2DC_EVPowerDeliveryParameterType(stream,&PowerDeliveryReqType.DC_EVPowerDeliveryParameter.unwrap())?;
  
                      grammar_id = 3;
              }
              else if PowerDeliveryReqType.EVPowerDeliveryParameter.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Abstract element or type: START (EVPowerDeliveryParameterType); next=3
  
  
   
                      encode_ISO2EVPowerDeliveryParameterType(stream,&PowerDeliveryReqType.EVPowerDeliveryParameter.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateInstallationReq; type={urn:iso:15118:2:2013:MsgBody}CertificateInstallationReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); OEMProvisioningCert, certificateType (1, 1); ListOfRootCertificateIDs, ListOfRootCertificateIDsType (1, 1);
  pub fn encode_ISO2CertificateInstallationReqType(stream: &mut ExiBitstream, CertificateInstallationReqType: &ISO2CertificateInstallationReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 352;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           352=>{
               // Grammar: ID=352; read/write bits=1; START (Id)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (NCName); next=353
  
              // string should not be found in table, so add 2
  
               stream.write_u16(CertificateInstallationReqType.Id.len() as u16)?;
              stream.write_characters(&CertificateInstallationReqType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
  
              grammar_id = 353;
  }
           353=>{
               // Grammar: ID=353; read/write bits=1; START (OEMProvisioningCert)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (base64Binary); next=354
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(CertificateInstallationReqType.OEMProvisioningCert.len() as u16)?;
                          stream.write_bytes( &CertificateInstallationReqType.OEMProvisioningCert)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 354;
  
  
  }
           354=>{
               // Grammar: ID=354; read/write bits=1; START (ListOfRootCertificateIDs)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (ListOfRootCertificateIDsType); next=3
  
  
  
                  encode_ISO2ListOfRootCertificateIDsType(stream,&CertificateInstallationReqType.ListOfRootCertificateIDs)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CurrentDemandReq; type={urn:iso:15118:2:2013:MsgBody}CurrentDemandReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1); EVTargetCurrent, PhysicalValueType (1, 1); EVMaximumVoltageLimit, PhysicalValueType (0, 1); EVMaximumCurrentLimit, PhysicalValueType (0, 1); EVMaximumPowerLimit, PhysicalValueType (0, 1); BulkChargingComplete, boolean (0, 1); ChargingComplete, boolean (1, 1); RemainingTimeToFullSoC, PhysicalValueType (0, 1); RemainingTimeToBulkSoC, PhysicalValueType (0, 1); EVTargetVoltage, PhysicalValueType (1, 1);
  pub fn encode_ISO2CurrentDemandReqType(stream: &mut ExiBitstream, CurrentDemandReqType: &ISO2CurrentDemandReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 355;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           355=>{
               // Grammar: ID=355; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVStatusType); next=356
  
  
  
                  encode_ISO2DC_EVStatusType(stream,&CurrentDemandReqType.DC_EVStatus)?;
  
                  grammar_id = 356;
  }
           356=>{
               // Grammar: ID=356; read/write bits=1; START (EVTargetCurrent)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=357
  
  
  
                  encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.EVTargetCurrent)?;
  
                  grammar_id = 357;
  }
           357=>{
               // Grammar: ID=357; read/write bits=3; START (EVMaximumVoltageLimit), START (EVMaximumCurrentLimit), START (EVMaximumPowerLimit), START (BulkChargingComplete), START (ChargingComplete)
              if CurrentDemandReqType.EVMaximumVoltageLimit.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (EVMaximumVoltageLimit, PhysicalValueType); next=358
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.EVMaximumVoltageLimit.unwrap())?;
  
                      grammar_id = 358;
              }
              else if CurrentDemandReqType.EVMaximumCurrentLimit.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (EVMaximumCurrentLimit, PhysicalValueType); next=359
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.EVMaximumCurrentLimit.unwrap())?;
  
                      grammar_id = 359;
              }
              else if CurrentDemandReqType.EVMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (EVMaximumPowerLimit, PhysicalValueType); next=360
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.EVMaximumPowerLimit.unwrap())?;
  
                      grammar_id = 360;
              }
              else if CurrentDemandReqType.BulkChargingComplete.is_some()
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (BulkChargingComplete, boolean); next=361
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.BulkChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 361;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 4 as u32)?;
                  // Event: START (ChargingComplete, boolean); next=362
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.ChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 362;
  
  
  
              }
  }
           358=>{
               // Grammar: ID=358; read/write bits=3; START (EVMaximumCurrentLimit), START (EVMaximumPowerLimit), START (BulkChargingComplete), START (ChargingComplete)
              if CurrentDemandReqType.EVMaximumCurrentLimit.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (EVMaximumCurrentLimit, PhysicalValueType); next=359
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.EVMaximumCurrentLimit.unwrap())?;
  
                      grammar_id = 359;
              }
              else if CurrentDemandReqType.EVMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (EVMaximumPowerLimit, PhysicalValueType); next=360
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.EVMaximumPowerLimit.unwrap())?;
  
                      grammar_id = 360;
              }
              else if CurrentDemandReqType.BulkChargingComplete.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (BulkChargingComplete, boolean); next=361
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.BulkChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 361;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (ChargingComplete, boolean); next=362
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.ChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 362;
  
  
  
              }
  }
           359=>{
               // Grammar: ID=359; read/write bits=2; START (EVMaximumPowerLimit), START (BulkChargingComplete), START (ChargingComplete)
              if CurrentDemandReqType.EVMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (EVMaximumPowerLimit, PhysicalValueType); next=360
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.EVMaximumPowerLimit.unwrap())?;
  
                      grammar_id = 360;
              }
              else if CurrentDemandReqType.BulkChargingComplete.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (BulkChargingComplete, boolean); next=361
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.BulkChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 361;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (ChargingComplete, boolean); next=362
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.ChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 362;
  
  
  
              }
  }
           360=>{
               // Grammar: ID=360; read/write bits=2; START (BulkChargingComplete), START (ChargingComplete)
              if CurrentDemandReqType.BulkChargingComplete.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (BulkChargingComplete, boolean); next=361
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.BulkChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 361;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (ChargingComplete, boolean); next=362
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.ChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 362;
  
  
  
              }
  }
           361=>{
               // Grammar: ID=361; read/write bits=1; START (ChargingComplete)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (boolean); next=362
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( CurrentDemandReqType.ChargingComplete)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 362;
  
  
  }
           362=>{
               // Grammar: ID=362; read/write bits=2; START (RemainingTimeToFullSoC), START (RemainingTimeToBulkSoC), START (EVTargetVoltage)
              if CurrentDemandReqType.RemainingTimeToFullSoC.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (RemainingTimeToFullSoC, PhysicalValueType); next=363
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.RemainingTimeToFullSoC.unwrap())?;
  
                      grammar_id = 363;
              }
              else if CurrentDemandReqType.RemainingTimeToBulkSoC.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (RemainingTimeToBulkSoC, PhysicalValueType); next=364
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.RemainingTimeToBulkSoC.unwrap())?;
  
                      grammar_id = 364;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (EVTargetVoltage, PhysicalValueType); next=3
  
  
  
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.EVTargetVoltage)?;
  
                      grammar_id = 3;
              }
  }
           363=>{
               // Grammar: ID=363; read/write bits=2; START (RemainingTimeToBulkSoC), START (EVTargetVoltage)
              if CurrentDemandReqType.RemainingTimeToBulkSoC.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (RemainingTimeToBulkSoC, PhysicalValueType); next=364
  
  
   
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.RemainingTimeToBulkSoC.unwrap())?;
  
                      grammar_id = 364;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (EVTargetVoltage, PhysicalValueType); next=3
  
  
  
                      encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.EVTargetVoltage)?;
  
                      grammar_id = 3;
              }
  }
           364=>{
               // Grammar: ID=364; read/write bits=1; START (EVTargetVoltage)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=3
  
  
  
                  encode_ISO2PhysicalValueType(stream,&CurrentDemandReqType.EVTargetVoltage)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryReq; type={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ServiceScope, serviceScopeType (0, 1); ServiceCategory, serviceCategoryType (0, 1);
  pub fn encode_ISO2ServiceDiscoveryReqType(stream: &mut ExiBitstream, ServiceDiscoveryReqType: &ISO2ServiceDiscoveryReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 365;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           365=>{
               // Grammar: ID=365; read/write bits=2; START (ServiceScope), START (ServiceCategory), END Element
              if ServiceDiscoveryReqType.ServiceScope.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ServiceScope, string); next=366
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
  
                       stream.write_u16(ServiceDiscoveryReqType.ServiceScope.expect("Value not Initialized").len() as u16)?;
                      stream.write_characters(&ServiceDiscoveryReqType.ServiceScope.expect("Value not Initialized").to_string(), ISO2ServiceScope_CHARACTER_SIZE)?;
  
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 366;
              }
              else if ServiceDiscoveryReqType.ServiceCategory.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (ServiceCategory, string); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(2, ServiceDiscoveryReqType.ServiceCategory as u32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           366=>{
               // Grammar: ID=366; read/write bits=2; START (ServiceCategory), END Element
              if ServiceDiscoveryReqType.ServiceCategory.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ServiceCategory, string); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(2, ServiceDiscoveryReqType.ServiceCategory as u32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: END Element; next=4
                  done = true;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PreChargeReq; type={urn:iso:15118:2:2013:MsgBody}PreChargeReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1); EVTargetVoltage, PhysicalValueType (1, 1); EVTargetCurrent, PhysicalValueType (1, 1);
  pub fn encode_ISO2PreChargeReqType(stream: &mut ExiBitstream, PreChargeReqType: &ISO2PreChargeReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 367;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           367=>{
               // Grammar: ID=367; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVStatusType); next=368
  
  
  
                  encode_ISO2DC_EVStatusType(stream,&PreChargeReqType.DC_EVStatus)?;
  
                  grammar_id = 368;
  }
           368=>{
               // Grammar: ID=368; read/write bits=1; START (EVTargetVoltage)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=369
  
  
  
                  encode_ISO2PhysicalValueType(stream,&PreChargeReqType.EVTargetVoltage)?;
  
                  grammar_id = 369;
  }
           369=>{
               // Grammar: ID=369; read/write bits=1; START (EVTargetCurrent)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (PhysicalValueType); next=3
  
  
  
                  encode_ISO2PhysicalValueType(stream,&PreChargeReqType.EVTargetCurrent)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeteringReceiptReq; type={urn:iso:15118:2:2013:MsgBody}MeteringReceiptReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (0, 1); SessionID, sessionIDType (1, 1); SAScheduleTupleID, SAIDType (0, 1); MeterInfo, MeterInfoType (1, 1);
  pub fn encode_ISO2MeteringReceiptReqType(stream: &mut ExiBitstream, MeteringReceiptReqType: &ISO2MeteringReceiptReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 370;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           370=>{
               // Grammar: ID=370; read/write bits=2; START (Id), START (SessionID)
              if MeteringReceiptReqType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (Id, NCName); next=371
  
                  // string should not be found in table, so add 2
  
                   stream.write_u16(MeteringReceiptReqType.Id.expect("Value not Initialized").len() as u16)?;
                  stream.write_characters(&MeteringReceiptReqType.Id.expect("Value not Initialized").to_string(), ISO2Id_CHARACTER_SIZE)?;
  
                  grammar_id = 371;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (SessionID, hexBinary); next=372
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16( MeteringReceiptReqType.SessionID.len() as u16)?;
                      stream.write_bytes( &MeteringReceiptReqType.SessionID)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 372;
  
  
  
  
              }
  }
           371=>{
               // Grammar: ID=371; read/write bits=1; START (SessionID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (hexBinary); next=372
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16( MeteringReceiptReqType.SessionID.len() as u16)?;
                  stream.write_bytes( &MeteringReceiptReqType.SessionID)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 372;
  
  
  
  }
           372=>{
               // Grammar: ID=372; read/write bits=2; START (SAScheduleTupleID), START (MeterInfo)
              if MeteringReceiptReqType.SAScheduleTupleID.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (SAScheduleTupleID, unsignedByte); next=373
  
  
                      stream.write_nbit_uint(1, 0)?; 
  
                      stream.write_nbit_uint(8, MeteringReceiptReqType.SAScheduleTupleID.unwrap() as u32 - 1)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
  
                      grammar_id = 373;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (MeterInfo, MeterInfoType); next=3
  
  
  
                      encode_ISO2MeterInfoType(stream,&MeteringReceiptReqType.MeterInfo)?;
  
                      grammar_id = 3;
              }
  }
           373=>{
               // Grammar: ID=373; read/write bits=1; START (MeterInfo)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (MeterInfoType); next=3
  
  
  
                  encode_ISO2MeterInfoType(stream,&MeteringReceiptReqType.MeterInfo)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionStopReq; type={urn:iso:15118:2:2013:MsgBody}SessionStopReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ChargingSession, chargingSessionType (1, 1);
  pub fn encode_ISO2SessionStopReqType(stream: &mut ExiBitstream, SessionStopReqType: &ISO2SessionStopReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 374;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           374=>{
               // Grammar: ID=374; read/write bits=1; START (ChargingSession)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(1, SessionStopReqType.ChargingSession as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PowerDeliveryRes; type={urn:iso:15118:2:2013:MsgBody}PowerDeliveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (0, 1); DC_EVSEStatus, DC_EVSEStatusType (0, 1); EVSEStatus, EVSEStatusType (0, 1);
  pub fn encode_ISO2PowerDeliveryResType(stream: &mut ExiBitstream, PowerDeliveryResType: &ISO2PowerDeliveryResType )->Result<(), BitstreamError> {
      let mut grammar_id = 375;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           375=>{
               // Grammar: ID=375; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=376
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, PowerDeliveryResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 376;
  }
           376=>{
               // Grammar: ID=376; read/write bits=2; START (AC_EVSEStatus), START (DC_EVSEStatus), START (EVSEStatus)
              if PowerDeliveryResType.AC_EVSEStatus.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (AC_EVSEStatus, EVSEStatusType); next=3
  
  
   
                      encode_ISO2AC_EVSEStatusType(stream,&PowerDeliveryResType.AC_EVSEStatus.unwrap())?;
  
                      grammar_id = 3;
              }
              else if PowerDeliveryResType.DC_EVSEStatus.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (DC_EVSEStatus, EVSEStatusType); next=3
  
  
   
                      encode_ISO2DC_EVSEStatusType(stream,&PowerDeliveryResType.DC_EVSEStatus.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (EVSEStatus, EVSEStatusType); next=3
  
  
  
                      encode_ISO2EVSEStatusType(stream,&PowerDeliveryResType.EVSEStatus)?;
  
                      grammar_id = 3;
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingStatusRes; type={urn:iso:15118:2:2013:MsgBody}ChargingStatusResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEID, evseIDType (1, 1); SAScheduleTupleID, SAIDType (1, 1); EVSEMaxCurrent, PhysicalValueType (0, 1); MeterInfo, MeterInfoType (0, 1); ReceiptRequired, boolean (0, 1); AC_EVSEStatus, AC_EVSEStatusType (1, 1);
  pub fn encode_ISO2ChargingStatusResType(stream: &mut ExiBitstream, ChargingStatusResType: &ISO2ChargingStatusResType )->Result<(), BitstreamError> {
      let mut grammar_id = 377;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           377=>{
               // Grammar: ID=377; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=378
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, ChargingStatusResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 378;
  }
           378=>{
               // Grammar: ID=378; read/write bits=1; START (EVSEID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=379
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
  
                   stream.write_u16(ChargingStatusResType.EVSEID.len() as u16)?;
                  stream.write_characters(&ChargingStatusResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
  
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 379;
  }
           379=>{
               // Grammar: ID=379; read/write bits=1; START (SAScheduleTupleID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (unsignedByte); next=380
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(8, ChargingStatusResType.SAScheduleTupleID as u32 - 1)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
  
                  grammar_id = 380;
  
  
  }
           380=>{
               // Grammar: ID=380; read/write bits=3; START (EVSEMaxCurrent), START (MeterInfo), START (ReceiptRequired), START (AC_EVSEStatus)
              if ChargingStatusResType.EVSEMaxCurrent.is_some()
              {
                  stream.write_nbit_uint(3, 0 as u32)?;
                  // Event: START (EVSEMaxCurrent, PhysicalValueType); next=381
  
  
   
                      encode_ISO2PhysicalValueType(stream,&ChargingStatusResType.EVSEMaxCurrent.unwrap())?;
  
                      grammar_id = 381;
              }
              else if ChargingStatusResType.MeterInfo.is_some()
              {
                  stream.write_nbit_uint(3, 1 as u32)?;
                  // Event: START (MeterInfo, MeterInfoType); next=382
  
  
   
                      encode_ISO2MeterInfoType(stream,&ChargingStatusResType.MeterInfo.unwrap())?;
  
                      grammar_id = 382;
              }
              else if ChargingStatusResType.ReceiptRequired.is_some()
              {
                  stream.write_nbit_uint(3, 2 as u32)?;
                  // Event: START (ReceiptRequired, boolean); next=383
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ChargingStatusResType.ReceiptRequired)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 383;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 3 as u32)?;
                  // Event: START (AC_EVSEStatus, EVSEStatusType); next=3
  
  
  
                      encode_ISO2AC_EVSEStatusType(stream,&ChargingStatusResType.AC_EVSEStatus)?;
  
                      grammar_id = 3;
              }
  }
           381=>{
               // Grammar: ID=381; read/write bits=2; START (MeterInfo), START (ReceiptRequired), START (AC_EVSEStatus)
              if ChargingStatusResType.MeterInfo.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (MeterInfo, MeterInfoType); next=382
  
  
   
                      encode_ISO2MeterInfoType(stream,&ChargingStatusResType.MeterInfo.unwrap())?;
  
                      grammar_id = 382;
              }
              else if ChargingStatusResType.ReceiptRequired.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (ReceiptRequired, boolean); next=383
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ChargingStatusResType.ReceiptRequired)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 383;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (AC_EVSEStatus, EVSEStatusType); next=3
  
  
  
                      encode_ISO2AC_EVSEStatusType(stream,&ChargingStatusResType.AC_EVSEStatus)?;
  
                      grammar_id = 3;
              }
  }
           382=>{
               // Grammar: ID=382; read/write bits=2; START (ReceiptRequired), START (AC_EVSEStatus)
              if ChargingStatusResType.ReceiptRequired.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (ReceiptRequired, boolean); next=383
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ChargingStatusResType.ReceiptRequired)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 383;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (AC_EVSEStatus, EVSEStatusType); next=3
  
  
  
                      encode_ISO2AC_EVSEStatusType(stream,&ChargingStatusResType.AC_EVSEStatus)?;
  
                      grammar_id = 3;
              }
  }
           383=>{
               // Grammar: ID=383; read/write bits=1; START (AC_EVSEStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVSEStatusType); next=3
  
  
  
                  encode_ISO2AC_EVSEStatusType(stream,&ChargingStatusResType.AC_EVSEStatus)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeteringReceiptRes; type={urn:iso:15118:2:2013:MsgBody}MeteringReceiptResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (0, 1); DC_EVSEStatus, DC_EVSEStatusType (0, 1); EVSEStatus, EVSEStatusType (0, 1);
  pub fn encode_ISO2MeteringReceiptResType(stream: &mut ExiBitstream, MeteringReceiptResType: &ISO2MeteringReceiptResType )->Result<(), BitstreamError> {
      let mut grammar_id = 384;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           384=>{
               // Grammar: ID=384; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=385
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, MeteringReceiptResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 385;
  }
           385=>{
               // Grammar: ID=385; read/write bits=2; START (AC_EVSEStatus), START (DC_EVSEStatus), START (EVSEStatus)
              if MeteringReceiptResType.AC_EVSEStatus.is_some()
              {
                  stream.write_nbit_uint(2, 0 as u32)?;
                  // Event: START (AC_EVSEStatus, EVSEStatusType); next=3
  
  
   
                      encode_ISO2AC_EVSEStatusType(stream,&MeteringReceiptResType.AC_EVSEStatus.unwrap())?;
  
                      grammar_id = 3;
              }
              else if MeteringReceiptResType.DC_EVSEStatus.is_some()
              {
                  stream.write_nbit_uint(2, 1 as u32)?;
                  // Event: START (DC_EVSEStatus, EVSEStatusType); next=3
  
  
   
                      encode_ISO2DC_EVSEStatusType(stream,&MeteringReceiptResType.DC_EVSEStatus.unwrap())?;
  
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 2 as u32)?;
                  // Event: START (EVSEStatus, EVSEStatusType); next=3
  
  
  
                      encode_ISO2EVSEStatusType(stream,&MeteringReceiptResType.EVSEStatus)?;
  
                      grammar_id = 3;
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CableCheckRes; type={urn:iso:15118:2:2013:MsgBody}CableCheckResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1);
  pub fn encode_ISO2CableCheckResType(stream: &mut ExiBitstream, CableCheckResType: &ISO2CableCheckResType )->Result<(), BitstreamError> {
      let mut grammar_id = 386;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           386=>{
               // Grammar: ID=386; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=387
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, CableCheckResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 387;
  }
           387=>{
               // Grammar: ID=387; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (EVSEStatusType); next=388
  
  
  
                  encode_ISO2DC_EVSEStatusType(stream,&CableCheckResType.DC_EVSEStatus)?;
  
                  grammar_id = 388;
  }
           388=>{
               // Grammar: ID=388; read/write bits=1; START (EVSEProcessing)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, CableCheckResType.EVSEProcessing as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateInstallationRes; type={urn:iso:15118:2:2013:MsgBody}CertificateInstallationResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); SAProvisioningCertificateChain, CertificateChainType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (1, 1); DHpublickey, DiffieHellmanPublickeyType (1, 1); eMAID, EMAIDType (1, 1);
  pub fn encode_ISO2CertificateInstallationResType(stream: &mut ExiBitstream, CertificateInstallationResType: &ISO2CertificateInstallationResType )->Result<(), BitstreamError> {
      let mut grammar_id = 389;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           389=>{
               // Grammar: ID=389; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=390
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, CertificateInstallationResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 390;
  }
           390=>{
               // Grammar: ID=390; read/write bits=1; START (SAProvisioningCertificateChain)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (CertificateChainType); next=391
  
  
  
                  encode_ISO2CertificateChainType(stream,&CertificateInstallationResType.SAProvisioningCertificateChain)?;
  
                  grammar_id = 391;
  }
           391=>{
               // Grammar: ID=391; read/write bits=1; START (ContractSignatureCertChain)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (CertificateChainType); next=392
  
  
  
                  encode_ISO2CertificateChainType(stream,&CertificateInstallationResType.ContractSignatureCertChain)?;
  
                  grammar_id = 392;
  }
           392=>{
               // Grammar: ID=392; read/write bits=1; START (ContractSignatureEncryptedPrivateKey)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (privateKeyType); next=393
  
  
  
                  encode_ISO2ContractSignatureEncryptedPrivateKeyType(stream,&CertificateInstallationResType.ContractSignatureEncryptedPrivateKey)?;
  
                  grammar_id = 393;
  }
           393=>{
               // Grammar: ID=393; read/write bits=1; START (DHpublickey)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (dHpublickeyType); next=394
  
  
  
                  encode_ISO2DiffieHellmanPublickeyType(stream,&CertificateInstallationResType.DHpublickey)?;
  
                  grammar_id = 394;
  }
           394=>{
               // Grammar: ID=394; read/write bits=1; START (eMAID)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (eMAIDType); next=3
  
  
  
                  encode_ISO2EMAIDType(stream,&CertificateInstallationResType.eMAID)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionStopRes; type={urn:iso:15118:2:2013:MsgBody}SessionStopResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1);
  pub fn encode_ISO2SessionStopResType(stream: &mut ExiBitstream, SessionStopResType: &ISO2SessionStopResType )->Result<(), BitstreamError> {
      let mut grammar_id = 395;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           395=>{
               // Grammar: ID=395; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, SessionStopResType.ResponseCode as u32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}Body; type={urn:iso:15118:2:2013:MsgBody}BodyType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: AuthorizationReq, AuthorizationReqType (0, 1); AuthorizationRes, AuthorizationResType (0, 1); BodyElement, BodyBaseType (0, 1); CableCheckReq, CableCheckReqType (0, 1); CableCheckRes, CableCheckResType (0, 1); CertificateInstallationReq, CertificateInstallationReqType (0, 1); CertificateInstallationRes, CertificateInstallationResType (0, 1); CertificateUpdateReq, CertificateUpdateReqType (0, 1); CertificateUpdateRes, CertificateUpdateResType (0, 1); ChargeParameterDiscoveryReq, ChargeParameterDiscoveryReqType (0, 1); ChargeParameterDiscoveryRes, ChargeParameterDiscoveryResType (0, 1); ChargingStatusReq, ChargingStatusReqType (0, 1); ChargingStatusRes, ChargingStatusResType (0, 1); CurrentDemandReq, CurrentDemandReqType (0, 1); CurrentDemandRes, CurrentDemandResType (0, 1); MeteringReceiptReq, MeteringReceiptReqType (0, 1); MeteringReceiptRes, MeteringReceiptResType (0, 1); PaymentDetailsReq, PaymentDetailsReqType (0, 1); PaymentDetailsRes, PaymentDetailsResType (0, 1); PaymentServiceSelectionReq, PaymentServiceSelectionReqType (0, 1); PaymentServiceSelectionRes, PaymentServiceSelectionResType (0, 1); PowerDeliveryReq, PowerDeliveryReqType (0, 1); PowerDeliveryRes, PowerDeliveryResType (0, 1); PreChargeReq, PreChargeReqType (0, 1); PreChargeRes, PreChargeResType (0, 1); ServiceDetailReq, ServiceDetailReqType (0, 1); ServiceDetailRes, ServiceDetailResType (0, 1); ServiceDiscoveryReq, ServiceDiscoveryReqType (0, 1); ServiceDiscoveryRes, ServiceDiscoveryResType (0, 1); SessionSetupReq, SessionSetupReqType (0, 1); SessionSetupRes, SessionSetupResType (0, 1); SessionStopReq, SessionStopReqType (0, 1); SessionStopRes, SessionStopResType (0, 1); WeldingDetectionReq, WeldingDetectionReqType (0, 1); WeldingDetectionRes, WeldingDetectionResType (0, 1);
  pub fn encode_ISO2BodyType(stream: &mut ExiBitstream, BodyType: &ISO2BodyType )->Result<(), BitstreamError> {
      let mut grammar_id = 396;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           396=>{
               // Grammar: ID=396; read/write bits=6; START (AuthorizationReq)
              if BodyType.AuthorizationReq.is_some()
              {
                  stream.write_nbit_uint(6, 0 as u32)?;
                  // Event: AuthorizationReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.AuthorizationRes.is_some()
              {
                  stream.write_nbit_uint(6, 1 as u32)?;
                  // Event: AuthorizationRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.BodyElement.is_some()
              {
                  stream.write_nbit_uint(6, 2 as u32)?;
                  // Event: BodyElement
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.CableCheckReq.is_some()
              {
                  stream.write_nbit_uint(6, 3 as u32)?;
                  // Event: CableCheckReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.CableCheckRes.is_some()
              {
                  stream.write_nbit_uint(6, 4 as u32)?;
                  // Event: CableCheckRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.CertificateInstallationReq.is_some()
              {
                  stream.write_nbit_uint(6, 5 as u32)?;
                  // Event: CertificateInstallationReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.CertificateInstallationRes.is_some()
              {
                  stream.write_nbit_uint(6, 6 as u32)?;
                  // Event: CertificateInstallationRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.CertificateUpdateReq.is_some()
              {
                  stream.write_nbit_uint(6, 7 as u32)?;
                  // Event: CertificateUpdateReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.CertificateUpdateRes.is_some()
              {
                  stream.write_nbit_uint(6, 8 as u32)?;
                  // Event: CertificateUpdateRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.ChargeParameterDiscoveryReq.is_some()
              {
                  stream.write_nbit_uint(6, 9 as u32)?;
                  // Event: ChargeParameterDiscoveryReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.ChargeParameterDiscoveryRes.is_some()
              {
                  stream.write_nbit_uint(6, 10 as u32)?;
                  // Event: ChargeParameterDiscoveryRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.ChargingStatusReq.is_some()
              {
                  stream.write_nbit_uint(6, 11 as u32)?;
                  // Event: ChargingStatusReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.ChargingStatusRes.is_some()
              {
                  stream.write_nbit_uint(6, 12 as u32)?;
                  // Event: ChargingStatusRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.CurrentDemandReq.is_some()
              {
                  stream.write_nbit_uint(6, 13 as u32)?;
                  // Event: CurrentDemandReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.CurrentDemandRes.is_some()
              {
                  stream.write_nbit_uint(6, 14 as u32)?;
                  // Event: CurrentDemandRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.MeteringReceiptReq.is_some()
              {
                  stream.write_nbit_uint(6, 15 as u32)?;
                  // Event: MeteringReceiptReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.MeteringReceiptRes.is_some()
              {
                  stream.write_nbit_uint(6, 16 as u32)?;
                  // Event: MeteringReceiptRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.PaymentDetailsReq.is_some()
              {
                  stream.write_nbit_uint(6, 17 as u32)?;
                  // Event: PaymentDetailsReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.PaymentDetailsRes.is_some()
              {
                  stream.write_nbit_uint(6, 18 as u32)?;
                  // Event: PaymentDetailsRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.PaymentServiceSelectionReq.is_some()
              {
                  stream.write_nbit_uint(6, 19 as u32)?;
                  // Event: PaymentServiceSelectionReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.PaymentServiceSelectionRes.is_some()
              {
                  stream.write_nbit_uint(6, 20 as u32)?;
                  // Event: PaymentServiceSelectionRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.PowerDeliveryReq.is_some()
              {
                  stream.write_nbit_uint(6, 21 as u32)?;
                  // Event: PowerDeliveryReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.PowerDeliveryRes.is_some()
              {
                  stream.write_nbit_uint(6, 22 as u32)?;
                  // Event: PowerDeliveryRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.PreChargeReq.is_some()
              {
                  stream.write_nbit_uint(6, 23 as u32)?;
                  // Event: PreChargeReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.PreChargeRes.is_some()
              {
                  stream.write_nbit_uint(6, 24 as u32)?;
                  // Event: PreChargeRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.ServiceDetailReq.is_some()
              {
                  stream.write_nbit_uint(6, 25 as u32)?;
                  // Event: ServiceDetailReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.ServiceDetailRes.is_some()
              {
                  stream.write_nbit_uint(6, 26 as u32)?;
                  // Event: ServiceDetailRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.ServiceDiscoveryReq.is_some()
              {
                  stream.write_nbit_uint(6, 27 as u32)?;
                  // Event: ServiceDiscoveryReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.ServiceDiscoveryRes.is_some()
              {
                  stream.write_nbit_uint(6, 28 as u32)?;
                  // Event: ServiceDiscoveryRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.SessionSetupReq.is_some()
              {
                  stream.write_nbit_uint(6, 29 as u32)?;
                  // Event: SessionSetupReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.SessionSetupRes.is_some()
              {
                  stream.write_nbit_uint(6, 30 as u32)?;
                  // Event: SessionSetupRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.SessionStopReq.is_some()
              {
                  stream.write_nbit_uint(6, 31 as u32)?;
                  // Event: SessionStopReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.SessionStopRes.is_some()
              {
                  stream.write_nbit_uint(6, 32 as u32)?;
                  // Event: SessionStopRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.WeldingDetectionReq.is_some()
              {
                  stream.write_nbit_uint(6, 33 as u32)?;
                  // Event: WeldingDetectionReq
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else if BodyType.WeldingDetectionRes.is_some()
              {
                  stream.write_nbit_uint(6, 34 as u32)?;
                  // Event: WeldingDetectionRes
  
  
  
                      //unsupported input
  
                      grammar_id = 3;
              }
              else
              {
                  return Err(BitstreamError::UNKNOWNEVENTFORENCODING);
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}V2G_Message; type=AnonymousType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Header, MessageHeaderType (1, 1); Body, BodyType (1, 1);
  pub fn encode_ISO2V2G_Message(stream: &mut ExiBitstream, V2G_Message: &ISO2V2G_Message )->Result<(), BitstreamError> {
      let mut grammar_id = 397;
      let mut done = false;
  
      while(!done)
      {
          match(grammar_id){
      
           397=>{
               // Grammar: ID=397; read/write bits=1; START (Header)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (MessageHeaderType); next=398
  
  
  
                  encode_ISO2MessageHeaderType(stream,&V2G_Message.Header)?;
  
                  grammar_id = 398;
  }
           398=>{
               // Grammar: ID=398; read/write bits=1; START (Body)
              stream.write_nbit_uint( 1, 0 as u32)?;
              // Event: START (BodyType); next=3
  
  
  
                  encode_ISO2BodyType(stream,&V2G_Message.Body)?;
  
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0 as u32)?;
              // Event: END Element; next=4
              done = true;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      Ok(())
      }
  
  
  
  
  // main function for encoding
  fn encode_ISO2exiDocument(stream: &mut ExiBitstream, exiDoc: &ISO2exiDocument)->Result<(),BitstreamError>{
      stream.write_header()?;
          else
          {
              return Err(BitstreamError::UNKNOWNEVENTFORENCODING);
          }
  }
  
  
  