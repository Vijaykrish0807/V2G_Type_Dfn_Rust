
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
use exigen_core::*;



 
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Cost; type={urn:iso:15118:2:2013:MsgDataTypes}CostType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: costKind, costKindType (1, 1); amount, unsignedInt (1, 1); amountMultiplier, unitMultiplierType (0, 1);
  pub fn encode_ISO2CostType(stream: &mut ExiBitstream, CostType: &ISO2CostType )->Result<(), BitstreamError> {
      let mut grammar_id = 0;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           0=>{
               // Grammar: ID=0; read/write bits=1; START (costKind)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=1
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, CostType.costKind)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 1;
  }
           1=>{
               // Grammar: ID=1; read/write bits=1; START (amount)
              stream.write_nbit_uint( 1, 0)?;
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
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (amountMultiplier, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(3, (CostType.amountMultiplier as u32 - -3))?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Transform; type={http://www.w3.org/2000/09/xmldsig#}TransformType; base type=; content type=mixed;
  //          abstract=False; final=False; choice=True;
  // Particle: Algorithm, anyURI (1, 1); ANY, anyType (0, 1); XPath, string (0, 1);
  pub fn encode_ISO2TransformType(stream: &mut ExiBitstream, TransformType: &ISO2TransformType )->Result<(), BitstreamError> {
      let mut grammar_id = 5;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           5=>{
               // Grammar: ID=5; read/write bits=1; START (Algorithm)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (anyURI); next=6
  
              // string should not be found in table, so add 2
               stream.write_u16((TransformType.Algorithm.len() as u16))?;
              stream.write_characters(&TransformType.Algorithm.to_string(), ISO2Algorithm_CHARACTER_SIZE)?;
              grammar_id = 6;
              }
              }
  }
           6=>{
               // Grammar: ID=6; read/write bits=3; START (XPath), START (ANY), END Element, START (ANY)
              if TransformType.XPath.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (XPath, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((TransformType.XPath.len() as u16))?;
                      stream.write_characters(&TransformType.XPath.to_string(), ISO2XPath_CHARACTER_SIZE)?;
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
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(TransformType.ANY.len() as u16)?;
                              stream.write_bytes( &TransformType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}TimeInterval; type={urn:iso:15118:2:2013:MsgDataTypes}IntervalType; base type=; content type=empty;
  //          abstract=True; final=False;
  fn encode_ISO2IntervalType(stream: &mut ExiBitstream,IntervalType: &struct_type)->Result<(),BitstreamError>{
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
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           7=>{
               // Grammar: ID=7; read/write bits=1; START (Transform)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (TransformType); next=8
  
  
                  encode_ISO2TransformType(stream, &TransformsType.Transform)?;
                  grammar_id = 8?;
  }
           8=>{
               // Grammar: ID=8; read/write bits=2; START (Transform), END Element
              if (1 == 0)
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Transform, TransformType); next=3
  
  
                      encode_ISO2TransformType(stream, &TransformsType.Transform)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}DSAKeyValue; type={http://www.w3.org/2000/09/xmldsig#}DSAKeyValueType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: P, CryptoBinary (0, 1)(was 1, 1)(seq. ['P', 'Q']); Q, CryptoBinary (0, 1)(was 1, 1)(seq. ['P', 'Q']); G, CryptoBinary (0, 1); Y, CryptoBinary (1, 1); J, CryptoBinary (0, 1); Seed, CryptoBinary (0, 1)(was 1, 1)(seq. ['Seed', 'PgenCounter']); PgenCounter, CryptoBinary (0, 1)(was 1, 1)(seq. ['Seed', 'PgenCounter']);
  pub fn encode_ISO2DSAKeyValueType(stream: &mut ExiBitstream, DSAKeyValueType: &ISO2DSAKeyValueType )->Result<(), BitstreamError> {
      let mut grammar_id = 9;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           9=>{
               // Grammar: ID=9; read/write bits=2; START (P), START (G), START (Y)
              if DSAKeyValueType.P.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (P, base64Binary); next=10
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.P.len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.P)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 10;
  
  
  
              }
              else if DSAKeyValueType.G.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (G, base64Binary); next=12
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.G.len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.G)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 12;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
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
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (base64Binary); next=11
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(DSAKeyValueType.Q.len() as u16)?;
                          stream.write_bytes( &DSAKeyValueType.Q)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 11;
  
  
  }
           11=>{
               // Grammar: ID=11; read/write bits=2; START (G), START (Y)
              if DSAKeyValueType.G.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (G, base64Binary); next=12
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.G.len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.G)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 12;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
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
              stream.write_nbit_uint( 1, 0)?;
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
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (J, base64Binary); next=14
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.J.len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.J)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 14;
  
  
  
              }
              else if DSAKeyValueType.Seed.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (Seed, base64Binary); next=15
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.Seed.len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.Seed)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 15;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           14=>{
               // Grammar: ID=14; read/write bits=2; START (Seed), END Element
              if DSAKeyValueType.Seed.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Seed, base64Binary); next=15
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.Seed.len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.Seed)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 15;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           15=>{
               // Grammar: ID=15; read/write bits=2; START (PgenCounter), END Element
              if DSAKeyValueType.PgenCounter.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (PgenCounter, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DSAKeyValueType.PgenCounter.len() as u16)?;
                              stream.write_bytes( &DSAKeyValueType.PgenCounter)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}X509IssuerSerial; type={http://www.w3.org/2000/09/xmldsig#}X509IssuerSerialType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: X509IssuerName, string (1, 1); X509SerialNumber, integer (1, 1);
  pub fn encode_ISO2X509IssuerSerialType(stream: &mut ExiBitstream, X509IssuerSerialType: &ISO2X509IssuerSerialType )->Result<(), BitstreamError> {
      let mut grammar_id = 16;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           16=>{
               // Grammar: ID=16; read/write bits=1; START (X509IssuerName)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=17
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
                   stream.write_u16((X509IssuerSerialType.X509IssuerName.len() as u16))?;
                  stream.write_characters(&X509IssuerSerialType.X509IssuerName.to_string(), ISO2X509IssuerName_CHARACTER_SIZE)?;
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 17;
  }
           17=>{
               // Grammar: ID=17; read/write bits=1; START (X509SerialNumber)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (decimal); next=3
  
  
                  stream.write_nbit_uint( 1, 0)?;
                  stream.write_i32(X509IssuerSerialType.X509SerialNumber as write_i32)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}RelativeTimeInterval; type={urn:iso:15118:2:2013:MsgDataTypes}RelativeTimeIntervalType; base type=IntervalType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: start, AnonType (1, 1); duration, AnonType (0, 1);
  pub fn encode_ISO2RelativeTimeIntervalType(stream: &mut ExiBitstream, RelativeTimeIntervalType: &ISO2RelativeTimeIntervalType )->Result<(), BitstreamError> {
      let mut grammar_id = 18;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           18=>{
               // Grammar: ID=18; read/write bits=1; START (start)
              stream.write_nbit_uint( 1, 0)?;
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
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (duration, unsignedInt); next=3
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_u32(RelativeTimeIntervalType.duration as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}DigestMethod; type={http://www.w3.org/2000/09/xmldsig#}DigestMethodType; base type=; content type=mixed;
  //          abstract=False; final=False;
  // Particle: Algorithm, anyURI (1, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2DigestMethodType(stream: &mut ExiBitstream, DigestMethodType: &ISO2DigestMethodType )->Result<(), BitstreamError> {
      let mut grammar_id = 20;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           20=>{
               // Grammar: ID=20; read/write bits=1; START (Algorithm)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (anyURI); next=21
  
              // string should not be found in table, so add 2
               stream.write_u16((DigestMethodType.Algorithm.len() as u16))?;
              stream.write_characters(&DigestMethodType.Algorithm.to_string(), ISO2Algorithm_CHARACTER_SIZE)?;
              grammar_id = 21;
              }
              }
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
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(DigestMethodType.ANY.len() as u16)?;
                              stream.write_bytes( &DigestMethodType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}RSAKeyValue; type={http://www.w3.org/2000/09/xmldsig#}RSAKeyValueType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Modulus, CryptoBinary (1, 1); Exponent, CryptoBinary (1, 1);
  pub fn encode_ISO2RSAKeyValueType(stream: &mut ExiBitstream, RSAKeyValueType: &ISO2RSAKeyValueType )->Result<(), BitstreamError> {
      let mut grammar_id = 22;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           22=>{
               // Grammar: ID=22; read/write bits=1; START (Modulus)
              stream.write_nbit_uint( 1, 0)?;
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
              stream.write_nbit_uint( 1, 0)?;
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
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}physicalValue; type={urn:iso:15118:2:2013:MsgDataTypes}PhysicalValueType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Multiplier, unitMultiplierType (1, 1); Unit, unitSymbolType (1, 1); Value, short (1, 1);
  pub fn encode_ISO2PhysicalValueType(stream: &mut ExiBitstream, PhysicalValueType: &ISO2PhysicalValueType )->Result<(), BitstreamError> {
      let mut grammar_id = 24;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           24=>{
               // Grammar: ID=24; read/write bits=1; START (Multiplier)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (byte); next=25
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(3, (PhysicalValueType.Multiplier as u32 - -3))?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 25;
  
  
  }
           25=>{
               // Grammar: ID=25; read/write bits=1; START (Unit)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=26
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(3, PhysicalValueType.Unit)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 26;
  }
           26=>{
               // Grammar: ID=26; read/write bits=1; START (Value)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (int); next=3
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_i16( PhysicalValueType.Value as i16)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ConsumptionCost; type={urn:iso:15118:2:2013:MsgDataTypes}ConsumptionCostType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: startValue, PhysicalValueType (1, 1); Cost, CostType (1, 3);
  pub fn encode_ISO2ConsumptionCostType(stream: &mut ExiBitstream, ConsumptionCostType: &ISO2ConsumptionCostType )->Result<(), BitstreamError> {
      let mut grammar_id = 27;
      let mut done = 0;
      let mut Cost_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           27=>{
               // Grammar: ID=27; read/write bits=1; START (startValue)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=28
  
  
                  encode_ISO2PhysicalValueType(stream, &ConsumptionCostType.startValue)?;
                  grammar_id = 28?;
  }
           28=>{
               // Grammar: ID=28; read/write bits=1; START (Cost)
              if (Cost_currentIndex < ConsumptionCostType.Cost.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (CostType); next=29
  
  
                      encode_ISO2CostType(stream, &ConsumptionCostType.Cost.array[Cost_currentIndex])?;
                      Cost_currentIndex+=1;
                      grammar_id = 29;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           29=>{
               // Grammar: ID=29; read/write bits=2; START (Cost), END Element
              if (Cost_currentIndex < ConsumptionCostType.Cost.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (CostType); next=30
  
  
                      encode_ISO2CostType(stream, &ConsumptionCostType.Cost.array[Cost_currentIndex])?;
                      Cost_currentIndex+=1;
                      grammar_id = 30;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           30=>{
               // Grammar: ID=30; read/write bits=2; START (Cost), END Element
              if (Cost_currentIndex < ConsumptionCostType.Cost.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (CostType); next=3
  
  
                      encode_ISO2CostType(stream, &ConsumptionCostType.Cost.array[Cost_currentIndex])?;
                      Cost_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleEntry; type={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleEntryType; base type=EntryType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: RelativeTimeInterval, RelativeTimeIntervalType (0, 1); TimeInterval, IntervalType (0, 1); PMax, PhysicalValueType (1, 1);
  pub fn encode_ISO2PMaxScheduleEntryType(stream: &mut ExiBitstream, PMaxScheduleEntryType: &ISO2PMaxScheduleEntryType )->Result<(), BitstreamError> {
      let mut grammar_id = 31;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           31=>{
               // Grammar: ID=31; read/write bits=2; START (RelativeTimeInterval), START (TimeInterval)
              if PMaxScheduleEntryType.RelativeTimeInterval.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (RelativeTimeInterval, IntervalType); next=32
  
  
                      encode_ISO2RelativeTimeIntervalType(stream, &PMaxScheduleEntryType.RelativeTimeInterval)?;
                      grammar_id = 32?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Abstract element or type: START (IntervalType); next=32
  
  
                      encode_ISO2IntervalType(stream, &PMaxScheduleEntryType.TimeInterval)?;
                      grammar_id = 32?;
              }
  }
           32=>{
               // Grammar: ID=32; read/write bits=1; START (PMax)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=3
  
  
                  encode_ISO2PhysicalValueType(stream, &PMaxScheduleEntryType.PMax)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffEntry; type={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffEntryType; base type=EntryType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: RelativeTimeInterval, RelativeTimeIntervalType (0, 1); TimeInterval, IntervalType (0, 1); EPriceLevel, unsignedByte (0, 1); ConsumptionCost, ConsumptionCostType (0, 3);
  pub fn encode_ISO2SalesTariffEntryType(stream: &mut ExiBitstream, SalesTariffEntryType: &ISO2SalesTariffEntryType )->Result<(), BitstreamError> {
      let mut grammar_id = 33;
      let mut done = 0;
      let mut ConsumptionCost_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           33=>{
               // Grammar: ID=33; read/write bits=2; START (RelativeTimeInterval), START (TimeInterval)
              if SalesTariffEntryType.RelativeTimeInterval.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (RelativeTimeInterval, IntervalType); next=34
  
  
                      encode_ISO2RelativeTimeIntervalType(stream, &SalesTariffEntryType.RelativeTimeInterval)?;
                      grammar_id = 34?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Abstract element or type: START (IntervalType); next=34
  
  
                      encode_ISO2IntervalType(stream, &SalesTariffEntryType.TimeInterval)?;
                      grammar_id = 34?;
              }
  }
           34=>{
               // Grammar: ID=34; read/write bits=2; START (EPriceLevel), START (ConsumptionCost), END Element
              if SalesTariffEntryType.EPriceLevel.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (EPriceLevel, unsignedShort); next=37
  
  
                      stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint( 8, SalesTariffEntryType.EPriceLevel)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 37;
  
  
  
              }
              else if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.arrayLen)
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=35 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost.array[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 35;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           35=>{
               // Grammar: ID=35; read/write bits=2; START (ConsumptionCost), END Element
              if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.arrayLen)
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=36 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost.array[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 36;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           36=>{
               // Grammar: ID=36; read/write bits=2; START (ConsumptionCost), END Element
              if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.arrayLen)
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=3 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost.array[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           37=>{
               // Grammar: ID=37; read/write bits=2; START (ConsumptionCost), END Element
              if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.arrayLen)
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=38 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost.array[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 38;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           38=>{
               // Grammar: ID=38; read/write bits=2; START (ConsumptionCost), END Element
              if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.arrayLen)
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=39 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost.array[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 39;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           39=>{
               // Grammar: ID=39; read/write bits=2; START (ConsumptionCost), END Element
              if (ConsumptionCost_currentIndex < SalesTariffEntryType.ConsumptionCost.arrayLen)
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ConsumptionCost, ConsumptionCostType); next=3 (optional array)
  
  
                      encode_ISO2ConsumptionCostType(stream, &SalesTariffEntryType.ConsumptionCost.array[ConsumptionCost_currentIndex])?;
                      ConsumptionCost_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}CanonicalizationMethod; type={http://www.w3.org/2000/09/xmldsig#}CanonicalizationMethodType; base type=; content type=mixed;
  //          abstract=False; final=False;
  // Particle: Algorithm, anyURI (1, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2CanonicalizationMethodType(stream: &mut ExiBitstream, CanonicalizationMethodType: &ISO2CanonicalizationMethodType )->Result<(), BitstreamError> {
      let mut grammar_id = 40;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           40=>{
               // Grammar: ID=40; read/write bits=1; START (Algorithm)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (anyURI); next=41
  
              // string should not be found in table, so add 2
               stream.write_u16((CanonicalizationMethodType.Algorithm.len() as u16))?;
              stream.write_characters(&CanonicalizationMethodType.Algorithm.to_string(), ISO2Algorithm_CHARACTER_SIZE)?;
              grammar_id = 41;
              }
              }
  }
           41=>{
               // Grammar: ID=41; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if CanonicalizationMethodType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(CanonicalizationMethodType.ANY.len() as u16)?;
                              stream.write_bytes( &CanonicalizationMethodType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignatureMethod; type={http://www.w3.org/2000/09/xmldsig#}SignatureMethodType; base type=; content type=mixed;
  //          abstract=False; final=False;
  // Particle: Algorithm, anyURI (1, 1); HMACOutputLength, HMACOutputLengthType (0, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2SignatureMethodType(stream: &mut ExiBitstream, SignatureMethodType: &ISO2SignatureMethodType )->Result<(), BitstreamError> {
      let mut grammar_id = 42;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           42=>{
               // Grammar: ID=42; read/write bits=1; START (Algorithm)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (anyURI); next=43
  
              // string should not be found in table, so add 2
               stream.write_u16((SignatureMethodType.Algorithm.len() as u16))?;
              stream.write_characters(&SignatureMethodType.Algorithm.to_string(), ISO2Algorithm_CHARACTER_SIZE)?;
              grammar_id = 43;
              }
              }
  }
           43=>{
               // Grammar: ID=43; read/write bits=3; START (HMACOutputLength), START (ANY), END Element, START (ANY)
              if SignatureMethodType.HMACOutputLength.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (HMACOutputLength, integer); next=44
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_i32(SignatureMethodType.HMACOutputLength as write_i32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 44;
  
  
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=1)
              //{
              // ***** //
              else if SignatureMethodType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SignatureMethodType.ANY.len() as u16)?;
                              stream.write_bytes( &SignatureMethodType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           44=>{
               // Grammar: ID=44; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if SignatureMethodType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SignatureMethodType.ANY.len() as u16)?;
                              stream.write_bytes( &SignatureMethodType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}KeyValue; type={http://www.w3.org/2000/09/xmldsig#}KeyValueType; base type=; content type=mixed;
  //          abstract=False; final=False; choice=True;
  // Particle: DSAKeyValue, DSAKeyValueType (0, 1); RSAKeyValue, RSAKeyValueType (0, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2KeyValueType(stream: &mut ExiBitstream, KeyValueType: &ISO2KeyValueType )->Result<(), BitstreamError> {
      let mut grammar_id = 45;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           45=>{
               // Grammar: ID=45; read/write bits=2; START (DSAKeyValue), START (RSAKeyValue), START (ANY)
              if KeyValueType.DSAKeyValue.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (DSAKeyValue, DSAKeyValueType); next=3
  
  
                      encode_ISO2DSAKeyValueType(stream, &KeyValueType.DSAKeyValue)?;
                      grammar_id = 3?;
              }
              else if KeyValueType.RSAKeyValue.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (RSAKeyValue, RSAKeyValueType); next=3
  
  
                      encode_ISO2RSAKeyValueType(stream, &KeyValueType.RSAKeyValue)?;
                      grammar_id = 3?;
              }
              else if KeyValueType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(KeyValueType.ANY.len() as u16)?;
                              stream.write_bytes( &KeyValueType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Parameter; type={urn:iso:15118:2:2013:MsgDataTypes}ParameterType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; choice=True;
  // Particle: Name, string (1, 1); boolValue, boolean (0, 1); byteValue, byte (0, 1); shortValue, short (0, 1); intValue, int (0, 1); physicalValue, PhysicalValueType (0, 1); stringValue, string (0, 1);
  pub fn encode_ISO2ParameterType(stream: &mut ExiBitstream, ParameterType: &ISO2ParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 46;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           46=>{
               // Grammar: ID=46; read/write bits=1; START (Name)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=47
  
              // string should not be found in table, so add 2
               stream.write_u16((ParameterType.Name.len() as u16))?;
              stream.write_characters(&ParameterType.Name.to_string(), ISO2Name_CHARACTER_SIZE)?;
              grammar_id = 47;
              }
              }
  }
           47=>{
               // Grammar: ID=47; read/write bits=3; START (boolValue), START (byteValue), START (shortValue), START (intValue), START (physicalValue), START (stringValue)
              if ParameterType.boolValue.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (boolValue, boolean); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ParameterType.boolValue)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  
              }
              else if ParameterType.byteValue.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
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
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (shortValue, int); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( ParameterType.shortValue as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
  
              }
              else if ParameterType.intValue.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (intValue, long); next=3
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_i32(ParameterType.intValue as write_i32)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
              }
              else if ParameterType.physicalValue.is_some()
              {
                  stream.write_nbit_uint(3, 4)?;
                  // Event: START (physicalValue, PhysicalValueType); next=3
  
  
                      encode_ISO2PhysicalValueType(stream, &ParameterType.physicalValue)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(3, 5)?;
                  // Event: START (stringValue, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((ParameterType.stringValue.len() as u16))?;
                      stream.write_characters(&ParameterType.stringValue.to_string(), ISO2stringValue_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}PMaxSchedule; type={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: PMaxScheduleEntry, PMaxScheduleEntryType (1, 1024);
  pub fn encode_ISO2PMaxScheduleType(stream: &mut ExiBitstream, PMaxScheduleType: &ISO2PMaxScheduleType )->Result<(), BitstreamError> {
      let mut grammar_id = 48;
      let mut done = 0;
      let mut PMaxScheduleEntry_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           48=>{
               // Grammar: ID=48; read/write bits=1; START (PMaxScheduleEntry)
              if (PMaxScheduleEntry_currentIndex < PMaxScheduleType.PMaxScheduleEntry.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (EntryType); next=49
  
  
                      encode_ISO2PMaxScheduleEntryType(stream, &PMaxScheduleType.PMaxScheduleEntry.array[PMaxScheduleEntry_currentIndex])?;
                      PMaxScheduleEntry_currentIndex+=1;
                      grammar_id = 49;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           49=>{
               // Grammar: ID=49; read/write bits=2; LOOP (PMaxScheduleEntry), END Element
              if (PMaxScheduleEntry_currentIndex < PMaxScheduleType.PMaxScheduleEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: LOOP (EntryType); next=4
  
  
                      encode_ISO2PMaxScheduleEntryType(stream, &PMaxScheduleType.PMaxScheduleEntry.array[PMaxScheduleEntry_currentIndex])?;
                      PMaxScheduleEntry_currentIndex+=1;
                      grammar_id = 4;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Reference; type={http://www.w3.org/2000/09/xmldsig#}ReferenceType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); Type, anyURI (0, 1); URI, anyURI (0, 1); Transforms, TransformsType (0, 1); DigestMethod, DigestMethodType (1, 1); DigestValue, DigestValueType (1, 1);
  pub fn encode_ISO2ReferenceType(stream: &mut ExiBitstream, ReferenceType: &ISO2ReferenceType )->Result<(), BitstreamError> {
      let mut grammar_id = 50;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           50=>{
               // Grammar: ID=50; read/write bits=3; START (Id), START (Type), START (URI), START (Transforms), START (DigestMethod)
              if ReferenceType.Id.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (Id, NCName); next=51
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ReferenceType.Id.len() as u16))?;
                  stream.write_characters(&ReferenceType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 51;
                  }
                  }
              }
              else if ReferenceType.Type.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (Type, anyURI); next=52
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ReferenceType.Type.len() as u16))?;
                  stream.write_characters(&ReferenceType.Type.to_string(), ISO2Type_CHARACTER_SIZE)?;
                  grammar_id = 52;
                  }
                  }
              }
              else if ReferenceType.URI.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (URI, anyURI); next=53
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ReferenceType.URI.len() as u16))?;
                  stream.write_characters(&ReferenceType.URI.to_string(), ISO2URI_CHARACTER_SIZE)?;
                  grammar_id = 53;
                  }
                  }
              }
              else if ReferenceType.Transforms.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (Transforms, TransformsType); next=54
  
  
                      encode_ISO2TransformsType(stream, &ReferenceType.Transforms)?;
                      grammar_id = 54?;
              }
              else
              {
                  stream.write_nbit_uint(3, 4)?;
                  // Event: START (DigestMethod, DigestMethodType); next=55
  
  
                      encode_ISO2DigestMethodType(stream, &ReferenceType.DigestMethod)?;
                      grammar_id = 55?;
              }
  }
           51=>{
               // Grammar: ID=51; read/write bits=3; START (Type), START (URI), START (Transforms), START (DigestMethod)
              if ReferenceType.Type.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (Type, anyURI); next=52
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ReferenceType.Type.len() as u16))?;
                  stream.write_characters(&ReferenceType.Type.to_string(), ISO2Type_CHARACTER_SIZE)?;
                  grammar_id = 52;
                  }
                  }
              }
              else if ReferenceType.URI.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (URI, anyURI); next=53
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ReferenceType.URI.len() as u16))?;
                  stream.write_characters(&ReferenceType.URI.to_string(), ISO2URI_CHARACTER_SIZE)?;
                  grammar_id = 53;
                  }
                  }
              }
              else if ReferenceType.Transforms.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (Transforms, TransformsType); next=54
  
  
                      encode_ISO2TransformsType(stream, &ReferenceType.Transforms)?;
                      grammar_id = 54?;
              }
              else
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (DigestMethod, DigestMethodType); next=55
  
  
                      encode_ISO2DigestMethodType(stream, &ReferenceType.DigestMethod)?;
                      grammar_id = 55?;
              }
  }
           52=>{
               // Grammar: ID=52; read/write bits=2; START (URI), START (Transforms), START (DigestMethod)
              if ReferenceType.URI.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (URI, anyURI); next=53
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ReferenceType.URI.len() as u16))?;
                  stream.write_characters(&ReferenceType.URI.to_string(), ISO2URI_CHARACTER_SIZE)?;
                  grammar_id = 53;
                  }
                  }
              }
              else if ReferenceType.Transforms.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (Transforms, TransformsType); next=54
  
  
                      encode_ISO2TransformsType(stream, &ReferenceType.Transforms)?;
                      grammar_id = 54?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (DigestMethod, DigestMethodType); next=55
  
  
                      encode_ISO2DigestMethodType(stream, &ReferenceType.DigestMethod)?;
                      grammar_id = 55?;
              }
  }
           53=>{
               // Grammar: ID=53; read/write bits=2; START (Transforms), START (DigestMethod)
              if ReferenceType.Transforms.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Transforms, TransformsType); next=54
  
  
                      encode_ISO2TransformsType(stream, &ReferenceType.Transforms)?;
                      grammar_id = 54?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (DigestMethod, DigestMethodType); next=55
  
  
                      encode_ISO2DigestMethodType(stream, &ReferenceType.DigestMethod)?;
                      grammar_id = 55?;
              }
  }
           54=>{
               // Grammar: ID=54; read/write bits=1; START (DigestMethod)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (DigestMethodType); next=55
  
  
                  encode_ISO2DigestMethodType(stream, &ReferenceType.DigestMethod)?;
                  grammar_id = 55?;
  }
           55=>{
               // Grammar: ID=55; read/write bits=1; START (DigestValue)
              stream.write_nbit_uint( 1, 0)?;
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
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}RetrievalMethod; type={http://www.w3.org/2000/09/xmldsig#}RetrievalMethodType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Type, anyURI (0, 1); URI, anyURI (0, 1); Transforms, TransformsType (0, 1);
  pub fn encode_ISO2RetrievalMethodType(stream: &mut ExiBitstream, RetrievalMethodType: &ISO2RetrievalMethodType )->Result<(), BitstreamError> {
      let mut grammar_id = 56;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           56=>{
               // Grammar: ID=56; read/write bits=3; START (Type), START (URI), START (Transforms), END Element
              if RetrievalMethodType.Type.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (Type, anyURI); next=57
  
                  // string should not be found in table, so add 2
                   stream.write_u16((RetrievalMethodType.Type.len() as u16))?;
                  stream.write_characters(&RetrievalMethodType.Type.to_string(), ISO2Type_CHARACTER_SIZE)?;
                  grammar_id = 57;
                  }
                  }
              }
              else if RetrievalMethodType.URI.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (URI, anyURI); next=58
  
                  // string should not be found in table, so add 2
                   stream.write_u16((RetrievalMethodType.URI.len() as u16))?;
                  stream.write_characters(&RetrievalMethodType.URI.to_string(), ISO2URI_CHARACTER_SIZE)?;
                  grammar_id = 58;
                  }
                  }
              }
              else if RetrievalMethodType.Transforms.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (Transforms, TransformsType); next=3
  
  
                      encode_ISO2TransformsType(stream, &RetrievalMethodType.Transforms)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           57=>{
               // Grammar: ID=57; read/write bits=2; START (URI), START (Transforms), END Element
              if RetrievalMethodType.URI.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (URI, anyURI); next=58
  
                  // string should not be found in table, so add 2
                   stream.write_u16((RetrievalMethodType.URI.len() as u16))?;
                  stream.write_characters(&RetrievalMethodType.URI.to_string(), ISO2URI_CHARACTER_SIZE)?;
                  grammar_id = 58;
                  }
                  }
              }
              else if RetrievalMethodType.Transforms.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (Transforms, TransformsType); next=3
  
  
                      encode_ISO2TransformsType(stream, &RetrievalMethodType.Transforms)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           58=>{
               // Grammar: ID=58; read/write bits=2; START (Transforms), END Element
              if RetrievalMethodType.Transforms.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Transforms, TransformsType); next=3
  
  
                      encode_ISO2TransformsType(stream, &RetrievalMethodType.Transforms)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SalesTariff; type={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); SalesTariffID, SAIDType (1, 1); SalesTariffDescription, tariffDescriptionType (0, 1); NumEPriceLevels, unsignedByte (0, 1); SalesTariffEntry, SalesTariffEntryType (1, 1024);
  pub fn encode_ISO2SalesTariffType(stream: &mut ExiBitstream, SalesTariffType: &ISO2SalesTariffType )->Result<(), BitstreamError> {
      let mut grammar_id = 59;
      let mut done = 0;
      let mut SalesTariffEntry_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           59=>{
               // Grammar: ID=59; read/write bits=2; START (Id), START (SalesTariffID)
              if SalesTariffType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Id, NCName); next=60
  
                  // string should not be found in table, so add 2
                   stream.write_u16((SalesTariffType.Id.len() as u16))?;
                  stream.write_characters(&SalesTariffType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 60;
                  }
                  }
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (SalesTariffID, unsignedByte); next=61
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(8, (SalesTariffType.SalesTariffID as u32 - 1))?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 61;
  
  
  
              }
  }
           60=>{
               // Grammar: ID=60; read/write bits=1; START (SalesTariffID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedByte); next=61
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(8, (SalesTariffType.SalesTariffID as u32 - 1))?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 61;
  
  
  }
           61=>{
               // Grammar: ID=61; read/write bits=2; START (SalesTariffDescription), START (NumEPriceLevels), START (SalesTariffEntry)
              if SalesTariffType.SalesTariffDescription.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (SalesTariffDescription, string); next=63
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((SalesTariffType.SalesTariffDescription.len() as u16))?;
                      stream.write_characters(&SalesTariffType.SalesTariffDescription.to_string(), ISO2SalesTariffDescription_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 63;
              }
              else if SalesTariffType.NumEPriceLevels.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (NumEPriceLevels, unsignedShort); next=65
  
  
                      stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint( 8, SalesTariffType.NumEPriceLevels)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 65;
  
  
  
              }
              else
              {
                  if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
                  {
                      stream.write_nbit_uint(2,2)?;
                      // Event: START (EntryType); next=3
  
  
                          encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry.array[SalesTariffEntry_currentIndex])?;
                          SalesTariffEntry_currentIndex+=1;
                          grammar_id = 3;
  
                  }
              }
  }
           62=>{
               // Grammar: ID=62; read/write bits=2; LOOP (SalesTariffEntry), END Element
              if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: LOOP (EntryType); next=4
  
  
                      encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry.array[SalesTariffEntry_currentIndex])?;
                      SalesTariffEntry_currentIndex+=1;
                      grammar_id = 4;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           63=>{
               // Grammar: ID=63; read/write bits=2; START (NumEPriceLevels), START (SalesTariffEntry)
              if SalesTariffType.NumEPriceLevels.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (NumEPriceLevels, unsignedShort); next=65
  
  
                      stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint( 8, SalesTariffType.NumEPriceLevels)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 65;
  
  
  
              }
              else
              {
                  if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
                  {
                      stream.write_nbit_uint(2,1)?;
                      // Event: START (EntryType); next=3
  
  
                          encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry.array[SalesTariffEntry_currentIndex])?;
                          SalesTariffEntry_currentIndex+=1;
                          grammar_id = 3;
  
                  }
              }
  }
           64=>{
               // Grammar: ID=64; read/write bits=2; LOOP (SalesTariffEntry), END Element
              if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: LOOP (EntryType); next=4
  
  
                      encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry.array[SalesTariffEntry_currentIndex])?;
                      SalesTariffEntry_currentIndex+=1;
                      grammar_id = 4;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           65=>{
               // Grammar: ID=65; read/write bits=1; START (SalesTariffEntry)
              if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (EntryType); next=66
  
  
                      encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry.array[SalesTariffEntry_currentIndex])?;
                      SalesTariffEntry_currentIndex+=1;
                      grammar_id = 66;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           66=>{
               // Grammar: ID=66; read/write bits=2; LOOP (SalesTariffEntry), END Element
              if (SalesTariffEntry_currentIndex < SalesTariffType.SalesTariffEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: LOOP (EntryType); next=4
  
  
                      encode_ISO2SalesTariffEntryType(stream, &SalesTariffType.SalesTariffEntry.array[SalesTariffEntry_currentIndex])?;
                      SalesTariffEntry_currentIndex+=1;
                      grammar_id = 4;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}X509Data; type={http://www.w3.org/2000/09/xmldsig#}X509DataType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: X509IssuerSerial, X509IssuerSerialType (0, 1); X509SKI, base64Binary (0, 1); X509SubjectName, string (0, 1); X509Certificate, base64Binary (0, 1); X509CRL, base64Binary (0, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2X509DataType(stream: &mut ExiBitstream, X509DataType: &ISO2X509DataType )->Result<(), BitstreamError> {
      let mut grammar_id = 67;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           67=>{
               // Grammar: ID=67; read/write bits=3; START (X509IssuerSerial), START (X509SKI), START (X509SubjectName), START (X509Certificate), START (X509CRL), START (ANY)
              if X509DataType.X509IssuerSerial.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (X509IssuerSerial, X509IssuerSerialType); next=3
  
  
                      encode_ISO2X509IssuerSerialType(stream, &X509DataType.X509IssuerSerial)?;
                      grammar_id = 3?;
              }
              else if X509DataType.X509SKI.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (X509SKI, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(X509DataType.X509SKI.len() as u16)?;
                              stream.write_bytes( &X509DataType.X509SKI)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else if X509DataType.X509SubjectName.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (X509SubjectName, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((X509DataType.X509SubjectName.len() as u16))?;
                      stream.write_characters(&X509DataType.X509SubjectName.to_string(), ISO2X509SubjectName_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else if X509DataType.X509Certificate.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (X509Certificate, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(X509DataType.X509Certificate.len() as u16)?;
                              stream.write_bytes( &X509DataType.X509Certificate)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else if X509DataType.X509CRL.is_some()
              {
                  stream.write_nbit_uint(3, 4)?;
                  // Event: START (X509CRL, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(X509DataType.X509CRL.len() as u16)?;
                              stream.write_bytes( &X509DataType.X509CRL)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else if X509DataType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 5)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(X509DataType.ANY.len() as u16)?;
                              stream.write_bytes( &X509DataType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}PGPData; type={http://www.w3.org/2000/09/xmldsig#}PGPDataType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; choice=True; sequence=True (2;
  // Particle: PGPKeyID, base64Binary (1, 1); PGPKeyPacket, base64Binary (0, 1); ANY, anyType (0, 1); PGPKeyPacket, base64Binary (1, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2PGPDataType(stream: &mut ExiBitstream, PGPDataType: &ISO2PGPDataType )->Result<(), BitstreamError> {
      let mut grammar_id = 68;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           68=>{
               // Grammar: ID=68; read/write bits=2; START (PGPKeyID), START (PGPKeyPacket)
              if PGPDataType.choice_1.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (PGPKeyID, base64Binary); next=69
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_1.PGPKeyID.len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_1.PGPKeyID)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 69;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (PGPKeyPacket, base64Binary); next=70
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_1.PGPKeyPacket.len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_1.PGPKeyPacket)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 70;
  
  
  
              }
  }
           69=>{
               // Grammar: ID=69; read/write bits=3; START (PGPKeyPacket), START (ANY), END Element, START (ANY)
              if PGPDataType.choice_1.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (PGPKeyPacket, base64Binary); next=70
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_1.PGPKeyPacket.len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_1.PGPKeyPacket)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 70;
  
  
  
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=1)
              //{
              // ***** //
              else if PGPDataType.choice_1.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (ANY, base64Binary); next=71
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_1.ANY.len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_1.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 71;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           70=>{
               // Grammar: ID=70; read/write bits=3; START (ANY), END Element, END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if PGPDataType.choice_1.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (ANY, base64Binary); next=71
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_1.ANY.len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_1.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 71;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           71=>{
               // Grammar: ID=71; read/write bits=1; START (PGPKeyPacket)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (base64Binary); next=72
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(PGPDataType.choice_2.PGPKeyPacket.len() as u16)?;
                          stream.write_bytes( &PGPDataType.choice_2.PGPKeyPacket)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 72;
  
  
  }
           72=>{
               // Grammar: ID=72; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if PGPDataType.choice_2.is_some()
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (ANY, base64Binary); next=71
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(PGPDataType.choice_2.ANY.len() as u16)?;
                              stream.write_bytes( &PGPDataType.choice_2.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 71;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SPKIData; type={http://www.w3.org/2000/09/xmldsig#}SPKIDataType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SPKISexp, base64Binary (1, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2SPKIDataType(stream: &mut ExiBitstream, SPKIDataType: &ISO2SPKIDataType )->Result<(), BitstreamError> {
      let mut grammar_id = 73;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           73=>{
               // Grammar: ID=73; read/write bits=1; START (SPKISexp)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (base64Binary); next=74
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(SPKIDataType.SPKISexp.len() as u16)?;
                          stream.write_bytes( &SPKIDataType.SPKISexp)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 74;
  
  
  }
           74=>{
               // Grammar: ID=74; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if SPKIDataType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SPKIDataType.ANY.len() as u16)?;
                              stream.write_bytes( &SPKIDataType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignedInfo; type={http://www.w3.org/2000/09/xmldsig#}SignedInfoType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); CanonicalizationMethod, CanonicalizationMethodType (1, 1); SignatureMethod, SignatureMethodType (1, 1); Reference, ReferenceType (1, 4);
  pub fn encode_ISO2SignedInfoType(stream: &mut ExiBitstream, SignedInfoType: &ISO2SignedInfoType )->Result<(), BitstreamError> {
      let mut grammar_id = 75;
      let mut done = 0;
      let mut Reference_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           75=>{
               // Grammar: ID=75; read/write bits=2; START (Id), START (CanonicalizationMethod)
              if SignedInfoType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Id, NCName); next=76
  
                  // string should not be found in table, so add 2
                   stream.write_u16((SignedInfoType.Id.len() as u16))?;
                  stream.write_characters(&SignedInfoType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 76;
                  }
                  }
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (CanonicalizationMethod, CanonicalizationMethodType); next=77
  
  
                      encode_ISO2CanonicalizationMethodType(stream, &SignedInfoType.CanonicalizationMethod)?;
                      grammar_id = 77?;
              }
  }
           76=>{
               // Grammar: ID=76; read/write bits=1; START (CanonicalizationMethod)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (CanonicalizationMethodType); next=77
  
  
                  encode_ISO2CanonicalizationMethodType(stream, &SignedInfoType.CanonicalizationMethod)?;
                  grammar_id = 77?;
  }
           77=>{
               // Grammar: ID=77; read/write bits=1; START (SignatureMethod)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (SignatureMethodType); next=78
  
  
                  encode_ISO2SignatureMethodType(stream, &SignedInfoType.SignatureMethod)?;
                  grammar_id = 78?;
  }
           78=>{
               // Grammar: ID=78; read/write bits=1; START (Reference)
              if (Reference_currentIndex < SignedInfoType.Reference.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (ReferenceType); next=79
  
  
                      encode_ISO2ReferenceType(stream, &SignedInfoType.Reference.array[Reference_currentIndex])?;
                      Reference_currentIndex+=1;
                      grammar_id = 79;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           79=>{
               // Grammar: ID=79; read/write bits=2; START (Reference), END Element
              if (Reference_currentIndex < SignedInfoType.Reference.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ReferenceType); next=80
  
  
                      encode_ISO2ReferenceType(stream, &SignedInfoType.Reference.array[Reference_currentIndex])?;
                      Reference_currentIndex+=1;
                      grammar_id = 80;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           80=>{
               // Grammar: ID=80; read/write bits=2; START (Reference), END Element
              if (Reference_currentIndex < SignedInfoType.Reference.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ReferenceType); next=81
  
  
                      encode_ISO2ReferenceType(stream, &SignedInfoType.Reference.array[Reference_currentIndex])?;
                      Reference_currentIndex+=1;
                      grammar_id = 81;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           81=>{
               // Grammar: ID=81; read/write bits=2; START (Reference), END Element
              if (Reference_currentIndex < SignedInfoType.Reference.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ReferenceType); next=82
  
  
                      encode_ISO2ReferenceType(stream, &SignedInfoType.Reference.array[Reference_currentIndex])?;
                      Reference_currentIndex+=1;
                      grammar_id = 82;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           82=>{
               // Grammar: ID=82; read/write bits=2; START (Reference), END Element
              if (Reference_currentIndex < SignedInfoType.Reference.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ReferenceType); next=3
  
  
                      encode_ISO2ReferenceType(stream, &SignedInfoType.Reference.array[Reference_currentIndex])?;
                      Reference_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ParameterSet; type={urn:iso:15118:2:2013:MsgDataTypes}ParameterSetType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ParameterSetID, short (1, 1); Parameter, ParameterType (1, 16);
  pub fn encode_ISO2ParameterSetType(stream: &mut ExiBitstream, ParameterSetType: &ISO2ParameterSetType )->Result<(), BitstreamError> {
      let mut grammar_id = 83;
      let mut done = 0;
      let mut Parameter_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           83=>{
               // Grammar: ID=83; read/write bits=1; START (ParameterSetID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (int); next=84
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_i16( ParameterSetType.ParameterSetID as i16)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 84;
  
  
  
  }
           84=>{
               // Grammar: ID=84; read/write bits=1; START (Parameter)
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (ParameterType); next=85
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 85;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           85=>{
               // Grammar: ID=85; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=86
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 86;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           86=>{
               // Grammar: ID=86; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=87
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 87;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           87=>{
               // Grammar: ID=87; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=88
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 88;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           88=>{
               // Grammar: ID=88; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=89
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 89;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           89=>{
               // Grammar: ID=89; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=90
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 90;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           90=>{
               // Grammar: ID=90; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=91
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 91;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           91=>{
               // Grammar: ID=91; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=92
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 92;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           92=>{
               // Grammar: ID=92; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=93
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 93;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           93=>{
               // Grammar: ID=93; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=94
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 94;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           94=>{
               // Grammar: ID=94; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=95
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 95;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           95=>{
               // Grammar: ID=95; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=96
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 96;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           96=>{
               // Grammar: ID=96; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=97
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 97;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           97=>{
               // Grammar: ID=97; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=98
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 98;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           98=>{
               // Grammar: ID=98; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=99
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 99;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           99=>{
               // Grammar: ID=99; read/write bits=2; START (Parameter), END Element
              if (Parameter_currentIndex < ParameterSetType.Parameter.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ParameterType); next=3
  
  
                      encode_ISO2ParameterType(stream, &ParameterSetType.Parameter.array[Parameter_currentIndex])?;
                      Parameter_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SelectedService; type={urn:iso:15118:2:2013:MsgDataTypes}SelectedServiceType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ServiceID, serviceIDType (1, 1); ParameterSetID, short (0, 1);
  pub fn encode_ISO2SelectedServiceType(stream: &mut ExiBitstream, SelectedServiceType: &ISO2SelectedServiceType )->Result<(), BitstreamError> {
      let mut grammar_id = 100;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           100=>{
               // Grammar: ID=100; read/write bits=1; START (ServiceID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedShort); next=101
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(SelectedServiceType.ServiceID as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 101;
  
  
  }
           101=>{
               // Grammar: ID=101; read/write bits=2; START (ParameterSetID), END Element
              if SelectedServiceType.ParameterSetID.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ParameterSetID, int); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( SelectedServiceType.ParameterSetID as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Service; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ServiceID, serviceIDType (1, 1); ServiceName, serviceNameType (0, 1); ServiceCategory, serviceCategoryType (1, 1); ServiceScope, serviceScopeType (0, 1); FreeService, boolean (1, 1);
  pub fn encode_ISO2ServiceType(stream: &mut ExiBitstream, ServiceType: &ISO2ServiceType )->Result<(), BitstreamError> {
      let mut grammar_id = 102;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           102=>{
               // Grammar: ID=102; read/write bits=1; START (ServiceID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedShort); next=103
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(ServiceType.ServiceID as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 103;
  
  
  }
           103=>{
               // Grammar: ID=103; read/write bits=2; START (ServiceName), START (ServiceCategory)
              if ServiceType.ServiceName.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ServiceName, string); next=104
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((ServiceType.ServiceName.len() as u16))?;
                      stream.write_characters(&ServiceType.ServiceName.to_string(), ISO2ServiceName_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 104;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (ServiceCategory, string); next=105
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(2, ServiceType.ServiceCategory)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 105;
              }
  }
           104=>{
               // Grammar: ID=104; read/write bits=1; START (ServiceCategory)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=105
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, ServiceType.ServiceCategory)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 105;
  }
           105=>{
               // Grammar: ID=105; read/write bits=2; START (ServiceScope), START (FreeService)
              if ServiceType.ServiceScope.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ServiceScope, string); next=106
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((ServiceType.ServiceScope.len() as u16))?;
                      stream.write_characters(&ServiceType.ServiceScope.to_string(), ISO2ServiceScope_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 106;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (FreeService, boolean); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ServiceType.FreeService)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  
              }
  }
           106=>{
               // Grammar: ID=106; read/write bits=1; START (FreeService)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (boolean); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( ServiceType.FreeService)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleTuple; type={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleTupleType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SAScheduleTupleID, SAIDType (1, 1); PMaxSchedule, PMaxScheduleType (1, 1); SalesTariff, SalesTariffType (0, 1);
  pub fn encode_ISO2SAScheduleTupleType(stream: &mut ExiBitstream, SAScheduleTupleType: &ISO2SAScheduleTupleType )->Result<(), BitstreamError> {
      let mut grammar_id = 107;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           107=>{
               // Grammar: ID=107; read/write bits=1; START (SAScheduleTupleID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedByte); next=108
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(8, (SAScheduleTupleType.SAScheduleTupleID as u32 - 1))?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 108;
  
  
  }
           108=>{
               // Grammar: ID=108; read/write bits=1; START (PMaxSchedule)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PMaxScheduleType); next=109
  
  
                  encode_ISO2PMaxScheduleType(stream, &SAScheduleTupleType.PMaxSchedule)?;
                  grammar_id = 109?;
  }
           109=>{
               // Grammar: ID=109; read/write bits=2; START (SalesTariff), END Element
              if SAScheduleTupleType.SalesTariff.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (SalesTariff, SalesTariffType); next=3
  
  
                      encode_ISO2SalesTariffType(stream, &SAScheduleTupleType.SalesTariff)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ProfileEntry; type={urn:iso:15118:2:2013:MsgDataTypes}ProfileEntryType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ChargingProfileEntryStart, unsignedInt (1, 1); ChargingProfileEntryMaxPower, PhysicalValueType (1, 1); ChargingProfileEntryMaxNumberOfPhasesInUse, maxNumPhasesType (0, 1);
  pub fn encode_ISO2ProfileEntryType(stream: &mut ExiBitstream, ProfileEntryType: &ISO2ProfileEntryType )->Result<(), BitstreamError> {
      let mut grammar_id = 110;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           110=>{
               // Grammar: ID=110; read/write bits=1; START (ChargingProfileEntryStart)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedLong); next=111
  
  
                  stream.write_nbit_uint( 1, 0)?;
                  stream.write_u32(ProfileEntryType.ChargingProfileEntryStart as u32)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 111;
  
  
  }
           111=>{
               // Grammar: ID=111; read/write bits=1; START (ChargingProfileEntryMaxPower)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=112
  
  
                  encode_ISO2PhysicalValueType(stream, &ProfileEntryType.ChargingProfileEntryMaxPower)?;
                  grammar_id = 112?;
  }
           112=>{
               // Grammar: ID=112; read/write bits=2; START (ChargingProfileEntryMaxNumberOfPhasesInUse), END Element
              if ProfileEntryType.ChargingProfileEntryMaxNumberOfPhasesInUse.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ChargingProfileEntryMaxNumberOfPhasesInUse, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(2, (ProfileEntryType.ChargingProfileEntryMaxNumberOfPhasesInUse as u32 - 1))?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignatureValue; type={http://www.w3.org/2000/09/xmldsig#}SignatureValueType; base type=base64Binary; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (0, 1); CONTENT, SignatureValueType (1, 1);
  pub fn encode_ISO2SignatureValueType(stream: &mut ExiBitstream, SignatureValueType: &ISO2SignatureValueType )->Result<(), BitstreamError> {
      let mut grammar_id = 113;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           113=>{
               // Grammar: ID=113; read/write bits=2; START (Id), START (CONTENT)
              if SignatureValueType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Id, NCName); next=114
  
                  // string should not be found in table, so add 2
                   stream.write_u16((SignatureValueType.Id.len() as u16))?;
                  stream.write_characters(&SignatureValueType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 114;
                  }
                  }
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (CONTENT, base64Binary); next=3
                      stream.write_u16(SignatureValueType.CONTENT.len() as u16)?;
                      stream.write_bytes(&SignatureValueType.CONTENT)?;
                      grammar_id = 3;
  
              }
  }
           114=>{
               // Grammar: ID=114; read/write bits=1; START (CONTENT)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (base64Binary); next=3
                  stream.write_u16(SignatureValueType.CONTENT.len() as u16)?;
                  stream.write_bytes(&SignatureValueType.CONTENT)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SubCertificates; type={urn:iso:15118:2:2013:MsgDataTypes}SubCertificatesType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Certificate, certificateType (1, 4);
  pub fn encode_ISO2SubCertificatesType(stream: &mut ExiBitstream, SubCertificatesType: &ISO2SubCertificatesType )->Result<(), BitstreamError> {
      let mut grammar_id = 115;
      let mut done = 0;
      let mut Certificate_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           115=>{
               // Grammar: ID=115; read/write bits=1; START (Certificate)
              if (Certificate_currentIndex < SubCertificatesType.Certificate.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (base64Binary); next=116
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SubCertificatesType.Certificate[Certificate_currentIndex].len() as u16)?;
                              stream.write_bytes( &SubCertificatesType.Certificate[Certificate_currentIndex])?;
                                  Certificate_currentIndex+=1;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 116;
  
  
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           116=>{
               // Grammar: ID=116; read/write bits=2; START (Certificate), END Element
              if (Certificate_currentIndex < SubCertificatesType.Certificate.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (base64Binary); next=117
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SubCertificatesType.Certificate[Certificate_currentIndex].len() as u16)?;
                              stream.write_bytes( &SubCertificatesType.Certificate[Certificate_currentIndex])?;
                                  Certificate_currentIndex+=1;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 117;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           117=>{
               // Grammar: ID=117; read/write bits=2; START (Certificate), END Element
              if (Certificate_currentIndex < SubCertificatesType.Certificate.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (base64Binary); next=118
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(SubCertificatesType.Certificate[Certificate_currentIndex].len() as u16)?;
                              stream.write_bytes( &SubCertificatesType.Certificate[Certificate_currentIndex])?;
                                  Certificate_currentIndex+=1;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 118;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           118=>{
               // Grammar: ID=118; read/write bits=2; START (Certificate), END Element
              if (Certificate_currentIndex < SubCertificatesType.Certificate.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
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
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}KeyInfo; type={http://www.w3.org/2000/09/xmldsig#}KeyInfoType; base type=; content type=mixed;
  //          abstract=False; final=False; choice=True;
  // Particle: Id, ID (0, 1); KeyName, string (0, 1); KeyValue, KeyValueType (0, 1); RetrievalMethod, RetrievalMethodType (0, 1); X509Data, X509DataType (0, 1); PGPData, PGPDataType (0, 1); SPKIData, SPKIDataType (0, 1); MgmtData, string (0, 1); ANY, anyType (0, 1);
  pub fn encode_ISO2KeyInfoType(stream: &mut ExiBitstream, KeyInfoType: &ISO2KeyInfoType )->Result<(), BitstreamError> {
      let mut grammar_id = 119;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           119=>{
               // Grammar: ID=119; read/write bits=4; START (Id), START (KeyName), START (KeyValue), START (RetrievalMethod), START (X509Data), START (PGPData), START (SPKIData), START (MgmtData), START (ANY)
              if KeyInfoType.Id.is_some()
              {
                  stream.write_nbit_uint(4, 0)?;
                  // Event: START (Id, NCName); next=120
  
                  // string should not be found in table, so add 2
                   stream.write_u16((KeyInfoType.Id.len() as u16))?;
                  stream.write_characters(&KeyInfoType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 120;
                  }
                  }
              }
              else if KeyInfoType.KeyName.is_some()
              {
                  stream.write_nbit_uint(4, 1)?;
                  // Event: START (KeyName, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((KeyInfoType.KeyName.len() as u16))?;
                      stream.write_characters(&KeyInfoType.KeyName.to_string(), ISO2KeyName_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else if KeyInfoType.KeyValue.is_some()
              {
                  stream.write_nbit_uint(4, 2)?;
                  // Event: START (KeyValue, KeyValueType); next=3
  
  
                      encode_ISO2KeyValueType(stream, &KeyInfoType.KeyValue)?;
                      grammar_id = 3?;
              }
              else if KeyInfoType.RetrievalMethod.is_some()
              {
                  stream.write_nbit_uint(4, 3)?;
                  // Event: START (RetrievalMethod, RetrievalMethodType); next=3
  
  
                      encode_ISO2RetrievalMethodType(stream, &KeyInfoType.RetrievalMethod)?;
                      grammar_id = 3?;
              }
              else if KeyInfoType.X509Data.is_some()
              {
                  stream.write_nbit_uint(4, 4)?;
                  // Event: START (X509Data, X509DataType); next=3
  
  
                      encode_ISO2X509DataType(stream, &KeyInfoType.X509Data)?;
                      grammar_id = 3?;
              }
              else if KeyInfoType.PGPData.is_some()
              {
                  stream.write_nbit_uint(4, 5)?;
                  // Event: START (PGPData, PGPDataType); next=3
  
  
                      encode_ISO2PGPDataType(stream, &KeyInfoType.PGPData)?;
                      grammar_id = 3?;
              }
              else if KeyInfoType.SPKIData.is_some()
              {
                  stream.write_nbit_uint(4, 6)?;
                  // Event: START (SPKIData, SPKIDataType); next=3
  
  
                      encode_ISO2SPKIDataType(stream, &KeyInfoType.SPKIData)?;
                      grammar_id = 3?;
              }
              else if KeyInfoType.MgmtData.is_some()
              {
                  stream.write_nbit_uint(4, 7)?;
                  // Event: START (MgmtData, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((KeyInfoType.MgmtData.len() as u16))?;
                      stream.write_characters(&KeyInfoType.MgmtData.to_string(), ISO2MgmtData_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else if KeyInfoType.ANY.is_some()
              {
                  stream.write_nbit_uint(4, 8)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(KeyInfoType.ANY.len() as u16)?;
                              stream.write_bytes( &KeyInfoType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
  }
           120=>{
               // Grammar: ID=120; read/write bits=4; START (KeyName), START (KeyValue), START (RetrievalMethod), START (X509Data), START (PGPData), START (SPKIData), START (MgmtData), START (ANY)
              if KeyInfoType.KeyName.is_some()
              {
                  stream.write_nbit_uint(4, 0)?;
                  // Event: START (KeyName, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((KeyInfoType.KeyName.len() as u16))?;
                      stream.write_characters(&KeyInfoType.KeyName.to_string(), ISO2KeyName_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else if KeyInfoType.KeyValue.is_some()
              {
                  stream.write_nbit_uint(4, 1)?;
                  // Event: START (KeyValue, KeyValueType); next=3
  
  
                      encode_ISO2KeyValueType(stream, &KeyInfoType.KeyValue)?;
                      grammar_id = 3?;
              }
              else if KeyInfoType.RetrievalMethod.is_some()
              {
                  stream.write_nbit_uint(4, 2)?;
                  // Event: START (RetrievalMethod, RetrievalMethodType); next=3
  
  
                      encode_ISO2RetrievalMethodType(stream, &KeyInfoType.RetrievalMethod)?;
                      grammar_id = 3?;
              }
              else if KeyInfoType.X509Data.is_some()
              {
                  stream.write_nbit_uint(4, 3)?;
                  // Event: START (X509Data, X509DataType); next=3
  
  
                      encode_ISO2X509DataType(stream, &KeyInfoType.X509Data)?;
                      grammar_id = 3?;
              }
              else if KeyInfoType.PGPData.is_some()
              {
                  stream.write_nbit_uint(4, 4)?;
                  // Event: START (PGPData, PGPDataType); next=3
  
  
                      encode_ISO2PGPDataType(stream, &KeyInfoType.PGPData)?;
                      grammar_id = 3?;
              }
              else if KeyInfoType.SPKIData.is_some()
              {
                  stream.write_nbit_uint(4, 5)?;
                  // Event: START (SPKIData, SPKIDataType); next=3
  
  
                      encode_ISO2SPKIDataType(stream, &KeyInfoType.SPKIData)?;
                      grammar_id = 3?;
              }
              else if KeyInfoType.MgmtData.is_some()
              {
                  stream.write_nbit_uint(4, 6)?;
                  // Event: START (MgmtData, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((KeyInfoType.MgmtData.len() as u16))?;
                      stream.write_characters(&KeyInfoType.MgmtData.to_string(), ISO2MgmtData_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else if KeyInfoType.ANY.is_some()
              {
                  stream.write_nbit_uint(4, 7)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(KeyInfoType.ANY.len() as u16)?;
                              stream.write_bytes( &KeyInfoType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Object; type={http://www.w3.org/2000/09/xmldsig#}ObjectType; base type=; content type=mixed;
  //          abstract=False; final=False;
  // Particle: Encoding, anyURI (0, 1); Id, ID (0, 1); MimeType, string (0, 1); ANY, anyType (0, 1)(old 1, 1);
  pub fn encode_ISO2ObjectType(stream: &mut ExiBitstream, ObjectType: &ISO2ObjectType )->Result<(), BitstreamError> {
      let mut grammar_id = 121;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           121=>{
               // Grammar: ID=121; read/write bits=3; START (Encoding), START (Id), START (MimeType), START (ANY), END Element, START (ANY)
              if ObjectType.Encoding.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (Encoding, anyURI); next=122
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ObjectType.Encoding.len() as u16))?;
                  stream.write_characters(&ObjectType.Encoding.to_string(), ISO2Encoding_CHARACTER_SIZE)?;
                  grammar_id = 122;
                  }
                  }
              }
              else if ObjectType.Id.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (Id, NCName); next=123
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ObjectType.Id.len() as u16))?;
                  stream.write_characters(&ObjectType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 123;
                  }
                  }
              }
              else if ObjectType.MimeType.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (MimeType, string); next=124
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ObjectType.MimeType.len() as u16))?;
                  stream.write_characters(&ObjectType.MimeType.to_string(), ISO2MimeType_CHARACTER_SIZE)?;
                  grammar_id = 124;
                  }
                  }
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=3)
              //{
              // ***** //
              else if ObjectType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 5)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(ObjectType.ANY.len() as u16)?;
                              stream.write_bytes( &ObjectType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 4)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           122=>{
               // Grammar: ID=122; read/write bits=3; START (Id), START (MimeType), START (ANY), END Element, START (ANY)
              if ObjectType.Id.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (Id, NCName); next=123
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ObjectType.Id.len() as u16))?;
                  stream.write_characters(&ObjectType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 123;
                  }
                  }
              }
              else if ObjectType.MimeType.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (MimeType, string); next=124
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ObjectType.MimeType.len() as u16))?;
                  stream.write_characters(&ObjectType.MimeType.to_string(), ISO2MimeType_CHARACTER_SIZE)?;
                  grammar_id = 124;
                  }
                  }
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=2)
              //{
              // ***** //
              else if ObjectType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 4)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(ObjectType.ANY.len() as u16)?;
                              stream.write_bytes( &ObjectType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           123=>{
               // Grammar: ID=123; read/write bits=3; START (MimeType), START (ANY), END Element, START (ANY)
              if ObjectType.MimeType.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (MimeType, string); next=124
  
                  // string should not be found in table, so add 2
                   stream.write_u16((ObjectType.MimeType.len() as u16))?;
                  stream.write_characters(&ObjectType.MimeType.to_string(), ISO2MimeType_CHARACTER_SIZE)?;
                  grammar_id = 124;
                  }
                  }
              }
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=1)
              //{
              // ***** //
              else if ObjectType.ANY.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(ObjectType.ANY.len() as u16)?;
                              stream.write_bytes( &ObjectType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           124=>{
               // Grammar: ID=124; read/write bits=2; START (ANY), END Element, START (ANY)
              // ***** //
              //{
                  // No code for unsupported generic event: ANY (index=0)
              //{
              // ***** //
              if ObjectType.ANY.is_some()
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (ANY, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(ObjectType.ANY.len() as u16)?;
                              stream.write_bytes( &ObjectType.ANY)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SupportedEnergyTransferMode; type={urn:iso:15118:2:2013:MsgDataTypes}SupportedEnergyTransferModeType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: EnergyTransferMode, EnergyTransferModeType (1, 6);
  pub fn encode_ISO2SupportedEnergyTransferModeType(stream: &mut ExiBitstream, SupportedEnergyTransferModeType: &ISO2SupportedEnergyTransferModeType )->Result<(), BitstreamError> {
      let mut grammar_id = 125;
      let mut done = 0;
      let mut EnergyTransferMode_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           125=>{
               // Grammar: ID=125; read/write bits=1; START (EnergyTransferMode)
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (string); next=126
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode.array[EnergyTransferMode_currentIndex])?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 126;
  
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           126=>{
               // Grammar: ID=126; read/write bits=2; START (EnergyTransferMode), END Element
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (string); next=127
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode.array[EnergyTransferMode_currentIndex])?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 127;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           127=>{
               // Grammar: ID=127; read/write bits=2; START (EnergyTransferMode), END Element
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (string); next=128
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode.array[EnergyTransferMode_currentIndex])?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 128;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           128=>{
               // Grammar: ID=128; read/write bits=2; START (EnergyTransferMode), END Element
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (string); next=129
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode.array[EnergyTransferMode_currentIndex])?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 129;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           129=>{
               // Grammar: ID=129; read/write bits=2; START (EnergyTransferMode), END Element
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (string); next=130
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode.array[EnergyTransferMode_currentIndex])?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 130;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           130=>{
               // Grammar: ID=130; read/write bits=2; START (EnergyTransferMode), END Element
              if (EnergyTransferMode_currentIndex < SupportedEnergyTransferModeType.EnergyTransferMode.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (string); next=3
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(3, SupportedEnergyTransferModeType.EnergyTransferMode.array[EnergyTransferMode_currentIndex])?;
                      EnergyTransferMode_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}DC_EVStatus; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVStatusType; base type=EVStatusType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: EVReady, boolean (1, 1); EVErrorCode, DC_EVErrorCodeType (1, 1); EVRESSSOC, percentValueType (1, 1);
  pub fn encode_ISO2DC_EVStatusType(stream: &mut ExiBitstream, DC_EVStatusType: &ISO2DC_EVStatusType )->Result<(), BitstreamError> {
      let mut grammar_id = 131;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           131=>{
               // Grammar: ID=131; read/write bits=1; START (EVReady)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (boolean); next=132
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( DC_EVStatusType.EVReady)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 132;
  
  
  }
           132=>{
               // Grammar: ID=132; read/write bits=1; START (EVErrorCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=133
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(4, DC_EVStatusType.EVErrorCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 133;
  }
           133=>{
               // Grammar: ID=133; read/write bits=1; START (EVRESSSOC)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (byte); next=3
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(7, DC_EVStatusType.EVRESSSOC as u32)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}BodyElement; type={urn:iso:15118:2:2013:MsgBody}BodyBaseType; base type=; content type=empty;
  //          abstract=True; final=False;
  fn encode_ISO2BodyBaseType(stream: &mut ExiBitstream,BodyBaseType: &struct_type)->Result<(),BitstreamError>{
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
      let mut grammar_id = 134;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           134=>{
               // Grammar: ID=134; read/write bits=1; START (FaultCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=135
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, NotificationType.FaultCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 135;
  }
           135=>{
               // Grammar: ID=135; read/write bits=2; START (FaultMsg), END Element
              if NotificationType.FaultMsg.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (FaultMsg, string); next=3
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((NotificationType.FaultMsg.len() as u16))?;
                      stream.write_characters(&NotificationType.FaultMsg.to_string(), ISO2FaultMsg_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}DC_EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEStatusType; base type=EVSEStatusType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); EVSEIsolationStatus, isolationLevelType (0, 1); EVSEStatusCode, DC_EVSEStatusCodeType (1, 1);
  pub fn encode_ISO2DC_EVSEStatusType(stream: &mut ExiBitstream, DC_EVSEStatusType: &ISO2DC_EVSEStatusType )->Result<(), BitstreamError> {
      let mut grammar_id = 136;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           136=>{
               // Grammar: ID=136; read/write bits=1; START (NotificationMaxDelay)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedInt); next=137
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(DC_EVSEStatusType.NotificationMaxDelay as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 137;
  
  
  }
           137=>{
               // Grammar: ID=137; read/write bits=1; START (EVSENotification)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=138
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, DC_EVSEStatusType.EVSENotification)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 138;
  }
           138=>{
               // Grammar: ID=138; read/write bits=2; START (EVSEIsolationStatus), START (EVSEStatusCode)
              if DC_EVSEStatusType.EVSEIsolationStatus.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (EVSEIsolationStatus, string); next=139
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(3, DC_EVSEStatusType.EVSEIsolationStatus)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 139;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (EVSEStatusCode, string); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(4, DC_EVSEStatusType.EVSEStatusCode)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
              }
  }
           139=>{
               // Grammar: ID=139; read/write bits=1; START (EVSEStatusCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(4, DC_EVSEStatusType.EVSEStatusCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEStatusType; base type=EVSEStatusType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); RCD, boolean (1, 1);
  pub fn encode_ISO2AC_EVSEStatusType(stream: &mut ExiBitstream, AC_EVSEStatusType: &ISO2AC_EVSEStatusType )->Result<(), BitstreamError> {
      let mut grammar_id = 140;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           140=>{
               // Grammar: ID=140; read/write bits=1; START (NotificationMaxDelay)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedInt); next=141
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(AC_EVSEStatusType.NotificationMaxDelay as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 141;
  
  
  }
           141=>{
               // Grammar: ID=141; read/write bits=1; START (EVSENotification)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=142
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, AC_EVSEStatusType.EVSENotification)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 142;
  }
           142=>{
               // Grammar: ID=142; read/write bits=1; START (RCD)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (boolean); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( AC_EVSEStatusType.RCD)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}EVSEStatusType; base type=; content type=ELEMENT-ONLY;
  //          abstract=True; final=False;
  // Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1);
  pub fn encode_ISO2EVSEStatusType(stream: &mut ExiBitstream, EVSEStatusType: &ISO2EVSEStatusType )->Result<(), BitstreamError> {
      let mut grammar_id = 143;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           143=>{
               // Grammar: ID=143; read/write bits=1; START (NotificationMaxDelay)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedInt); next=144
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(EVSEStatusType.NotificationMaxDelay as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 144;
  
  
  }
           144=>{
               // Grammar: ID=144; read/write bits=1; START (EVSENotification)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=145
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, EVSEStatusType.EVSENotification)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 145;
  }
           145=>{
               // Grammar: ID=145; read/write bits=1; START (AC_EVSEStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVSEStatusType); next=146
  
  
                  encode_ISO2AC_EVSEStatusType(stream, &EVSEStatusType.AC_EVSEStatus)?;
                  grammar_id = 146?;
  }
           146=>{
               // Grammar: ID=146; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVSEStatusType); next=3
  
  
                  encode_ISO2DC_EVSEStatusType(stream, &EVSEStatusType.DC_EVSEStatus)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SAProvisioningCertificateChain; type={urn:iso:15118:2:2013:MsgDataTypes}CertificateChainType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); Certificate, certificateType (1, 1); SubCertificates, SubCertificatesType (0, 1);
  pub fn encode_ISO2CertificateChainType(stream: &mut ExiBitstream, CertificateChainType: &ISO2CertificateChainType )->Result<(), BitstreamError> {
      let mut grammar_id = 147;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           147=>{
               // Grammar: ID=147; read/write bits=2; START (Id), START (Certificate)
              if CertificateChainType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Id, NCName); next=148
  
                  // string should not be found in table, so add 2
                   stream.write_u16((CertificateChainType.Id.len() as u16))?;
                  stream.write_characters(&CertificateChainType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 148;
                  }
                  }
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (Certificate, base64Binary); next=149
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(CertificateChainType.Certificate.len() as u16)?;
                              stream.write_bytes( &CertificateChainType.Certificate)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 149;
  
  
  
              }
  }
           148=>{
               // Grammar: ID=148; read/write bits=1; START (Certificate)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (base64Binary); next=149
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(CertificateChainType.Certificate.len() as u16)?;
                          stream.write_bytes( &CertificateChainType.Certificate)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 149;
  
  
  }
           149=>{
               // Grammar: ID=149; read/write bits=2; START (SubCertificates), END Element
              if CertificateChainType.SubCertificates.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (SubCertificates, SubCertificatesType); next=3
  
  
                      encode_ISO2SubCertificatesType(stream, &CertificateChainType.SubCertificates)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SelectedServiceList; type={urn:iso:15118:2:2013:MsgDataTypes}SelectedServiceListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SelectedService, SelectedServiceType (1, 16);
  pub fn encode_ISO2SelectedServiceListType(stream: &mut ExiBitstream, SelectedServiceListType: &ISO2SelectedServiceListType )->Result<(), BitstreamError> {
      let mut grammar_id = 150;
      let mut done = 0;
      let mut SelectedService_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           150=>{
               // Grammar: ID=150; read/write bits=1; START (SelectedService)
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (SelectedServiceType); next=151
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 151;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           151=>{
               // Grammar: ID=151; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=152
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 152;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           152=>{
               // Grammar: ID=152; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=153
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 153;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           153=>{
               // Grammar: ID=153; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=154
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 154;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           154=>{
               // Grammar: ID=154; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=155
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 155;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           155=>{
               // Grammar: ID=155; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=156
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 156;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           156=>{
               // Grammar: ID=156; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=157
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 157;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           157=>{
               // Grammar: ID=157; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=158
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 158;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           158=>{
               // Grammar: ID=158; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=159
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 159;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           159=>{
               // Grammar: ID=159; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=160
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 160;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           160=>{
               // Grammar: ID=160; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=161
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 161;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           161=>{
               // Grammar: ID=161; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=162
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 162;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           162=>{
               // Grammar: ID=162; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=163
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 163;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           163=>{
               // Grammar: ID=163; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=164
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 164;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           164=>{
               // Grammar: ID=164; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=165
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 165;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           165=>{
               // Grammar: ID=165; read/write bits=2; START (SelectedService), END Element
              if (SelectedService_currentIndex < SelectedServiceListType.SelectedService.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SelectedServiceType); next=3
  
  
                      encode_ISO2SelectedServiceType(stream, &SelectedServiceListType.SelectedService.array[SelectedService_currentIndex])?;
                      SelectedService_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentOptionList; type={urn:iso:15118:2:2013:MsgDataTypes}PaymentOptionListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: PaymentOption, paymentOptionType (1, 2);
  pub fn encode_ISO2PaymentOptionListType(stream: &mut ExiBitstream, PaymentOptionListType: &ISO2PaymentOptionListType )->Result<(), BitstreamError> {
      let mut grammar_id = 166;
      let mut done = 0;
      let mut PaymentOption_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           166=>{
               // Grammar: ID=166; read/write bits=1; START (PaymentOption)
              if (PaymentOption_currentIndex < PaymentOptionListType.PaymentOption.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (string); next=167
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(1, PaymentOptionListType.PaymentOption.array[PaymentOption_currentIndex])?;
                      PaymentOption_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 167;
  
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           167=>{
               // Grammar: ID=167; read/write bits=2; START (PaymentOption), END Element
              if (PaymentOption_currentIndex < PaymentOptionListType.PaymentOption.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (string); next=3
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_nbit_uint(1, PaymentOptionListType.PaymentOption.array[PaymentOption_currentIndex])?;
                      PaymentOption_currentIndex+=1;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ListOfRootCertificateIDs; type={urn:iso:15118:2:2013:MsgDataTypes}ListOfRootCertificateIDsType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: RootCertificateID, X509IssuerSerialType (1, 20);
  pub fn encode_ISO2ListOfRootCertificateIDsType(stream: &mut ExiBitstream, ListOfRootCertificateIDsType: &ISO2ListOfRootCertificateIDsType )->Result<(), BitstreamError> {
      let mut grammar_id = 168;
      let mut done = 0;
      let mut RootCertificateID_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           168=>{
               // Grammar: ID=168; read/write bits=1; START (RootCertificateID)
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (X509IssuerSerialType); next=169
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 169;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           169=>{
               // Grammar: ID=169; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=170
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 170;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           170=>{
               // Grammar: ID=170; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=171
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 171;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           171=>{
               // Grammar: ID=171; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=172
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 172;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           172=>{
               // Grammar: ID=172; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=173
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 173;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           173=>{
               // Grammar: ID=173; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=174
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 174;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           174=>{
               // Grammar: ID=174; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=175
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 175;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           175=>{
               // Grammar: ID=175; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=176
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 176;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           176=>{
               // Grammar: ID=176; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=177
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 177;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           177=>{
               // Grammar: ID=177; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=178
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 178;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           178=>{
               // Grammar: ID=178; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=179
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 179;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           179=>{
               // Grammar: ID=179; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=180
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 180;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           180=>{
               // Grammar: ID=180; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=181
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 181;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           181=>{
               // Grammar: ID=181; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=182
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 182;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           182=>{
               // Grammar: ID=182; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=183
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 183;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           183=>{
               // Grammar: ID=183; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=184
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 184;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           184=>{
               // Grammar: ID=184; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=185
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 185;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           185=>{
               // Grammar: ID=185; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=186
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 186;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           186=>{
               // Grammar: ID=186; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=187
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 187;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           187=>{
               // Grammar: ID=187; read/write bits=2; START (RootCertificateID), END Element
              if (RootCertificateID_currentIndex < ListOfRootCertificateIDsType.RootCertificateID.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (X509IssuerSerialType); next=3
  
  
                      encode_ISO2X509IssuerSerialType(stream, &ListOfRootCertificateIDsType.RootCertificateID.array[RootCertificateID_currentIndex])?;
                      RootCertificateID_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Signature; type={http://www.w3.org/2000/09/xmldsig#}SignatureType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); SignedInfo, SignedInfoType (1, 1); SignatureValue, SignatureValueType (1, 1); KeyInfo, KeyInfoType (0, 1); Object, ObjectType (0, 1);
  pub fn encode_ISO2SignatureType(stream: &mut ExiBitstream, SignatureType: &ISO2SignatureType )->Result<(), BitstreamError> {
      let mut grammar_id = 188;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           188=>{
               // Grammar: ID=188; read/write bits=2; START (Id), START (SignedInfo)
              if SignatureType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Id, NCName); next=189
  
                  // string should not be found in table, so add 2
                   stream.write_u16((SignatureType.Id.len() as u16))?;
                  stream.write_characters(&SignatureType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 189;
                  }
                  }
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (SignedInfo, SignedInfoType); next=190
  
  
                      encode_ISO2SignedInfoType(stream, &SignatureType.SignedInfo)?;
                      grammar_id = 190?;
              }
  }
           189=>{
               // Grammar: ID=189; read/write bits=1; START (SignedInfo)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (SignedInfoType); next=190
  
  
                  encode_ISO2SignedInfoType(stream, &SignatureType.SignedInfo)?;
                  grammar_id = 190?;
  }
           190=>{
               // Grammar: ID=190; read/write bits=1; START (SignatureValue)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (base64Binary); next=191
  
  
                  encode_ISO2SignatureValueType(stream, &SignatureType.SignatureValue)?;
                  grammar_id = 191?;
  }
           191=>{
               // Grammar: ID=191; read/write bits=2; START (KeyInfo), START (Object), END Element
              if SignatureType.KeyInfo.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (KeyInfo, KeyInfoType); next=193
  
  
                      encode_ISO2KeyInfoType(stream, &SignatureType.KeyInfo)?;
                      grammar_id = 193?;
              }
              else if SignatureType.Object.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (Object, ObjectType); next=192
  
  
                      encode_ISO2ObjectType(stream, &SignatureType.Object)?;
                      grammar_id = 192?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           192=>{
               // Grammar: ID=192; read/write bits=2; START (Object), END Element
              if (1 == 0)
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Object, ObjectType); next=3
  
  
                      encode_ISO2ObjectType(stream, &SignatureType.Object)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           193=>{
               // Grammar: ID=193; read/write bits=2; START (Object), END Element
              if SignatureType.Object.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Object, ObjectType); next=194
  
  
                      encode_ISO2ObjectType(stream, &SignatureType.Object)?;
                      grammar_id = 194?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           194=>{
               // Grammar: ID=194; read/write bits=2; START (Object), END Element
              if (1 == 0)
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Object, ObjectType); next=3
  
  
                      encode_ISO2ObjectType(stream, &SignatureType.Object)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceParameterList; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceParameterListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ParameterSet, ParameterSetType (1, 255);
  pub fn encode_ISO2ServiceParameterListType(stream: &mut ExiBitstream, ServiceParameterListType: &ISO2ServiceParameterListType )->Result<(), BitstreamError> {
      let mut grammar_id = 195;
      let mut done = 0;
      let mut ParameterSet_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           195=>{
               // Grammar: ID=195; read/write bits=1; START (ParameterSet)
              if (ParameterSet_currentIndex < ServiceParameterListType.ParameterSet.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (ParameterSetType); next=196
  
  
                      encode_ISO2ParameterSetType(stream, &ServiceParameterListType.ParameterSet.array[ParameterSet_currentIndex])?;
                      ParameterSet_currentIndex+=1;
                      grammar_id = 196;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           196=>{
               // Grammar: ID=196; read/write bits=2; LOOP (ParameterSet), END Element
              if (ParameterSet_currentIndex < ServiceParameterListType.ParameterSet.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: LOOP (ParameterSetType); next=4
  
  
                      encode_ISO2ParameterSetType(stream, &ServiceParameterListType.ParameterSet.array[ParameterSet_currentIndex])?;
                      ParameterSet_currentIndex+=1;
                      grammar_id = 4;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVChargeParameterType; base type=EVChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DepartureTime, unsignedInt (0, 1); DC_EVStatus, DC_EVStatusType (1, 1); EVMaximumCurrentLimit, PhysicalValueType (1, 1); EVMaximumPowerLimit, PhysicalValueType (0, 1); EVMaximumVoltageLimit, PhysicalValueType (1, 1); EVEnergyCapacity, PhysicalValueType (0, 1); EVEnergyRequest, PhysicalValueType (0, 1); FullSOC, percentValueType (0, 1); BulkSOC, percentValueType (0, 1);
  pub fn encode_ISO2DC_EVChargeParameterType(stream: &mut ExiBitstream, DC_EVChargeParameterType: &ISO2DC_EVChargeParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 197;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           197=>{
               // Grammar: ID=197; read/write bits=2; START (DepartureTime), START (DC_EVStatus)
              if DC_EVChargeParameterType.DepartureTime.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (DepartureTime, unsignedLong); next=198
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_u32(DC_EVChargeParameterType.DepartureTime as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 198;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (DC_EVStatus, EVStatusType); next=199
  
  
                      encode_ISO2DC_EVStatusType(stream, &DC_EVChargeParameterType.DC_EVStatus)?;
                      grammar_id = 199?;
              }
  }
           198=>{
               // Grammar: ID=198; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVStatusType); next=199
  
  
                  encode_ISO2DC_EVStatusType(stream, &DC_EVChargeParameterType.DC_EVStatus)?;
                  grammar_id = 199?;
  }
           199=>{
               // Grammar: ID=199; read/write bits=1; START (EVMaximumCurrentLimit)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=200
  
  
                  encode_ISO2PhysicalValueType(stream, &DC_EVChargeParameterType.EVMaximumCurrentLimit)?;
                  grammar_id = 200?;
  }
           200=>{
               // Grammar: ID=200; read/write bits=2; START (EVMaximumPowerLimit), START (EVMaximumVoltageLimit)
              if DC_EVChargeParameterType.EVMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (EVMaximumPowerLimit, PhysicalValueType); next=201
  
  
                      encode_ISO2PhysicalValueType(stream, &DC_EVChargeParameterType.EVMaximumPowerLimit)?;
                      grammar_id = 201?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (EVMaximumVoltageLimit, PhysicalValueType); next=202
  
  
                      encode_ISO2PhysicalValueType(stream, &DC_EVChargeParameterType.EVMaximumVoltageLimit)?;
                      grammar_id = 202?;
              }
  }
           201=>{
               // Grammar: ID=201; read/write bits=1; START (EVMaximumVoltageLimit)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=202
  
  
                  encode_ISO2PhysicalValueType(stream, &DC_EVChargeParameterType.EVMaximumVoltageLimit)?;
                  grammar_id = 202?;
  }
           202=>{
               // Grammar: ID=202; read/write bits=3; START (EVEnergyCapacity), START (EVEnergyRequest), START (FullSOC), START (BulkSOC), END Element
              if DC_EVChargeParameterType.EVEnergyCapacity.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (EVEnergyCapacity, PhysicalValueType); next=203
  
  
                      encode_ISO2PhysicalValueType(stream, &DC_EVChargeParameterType.EVEnergyCapacity)?;
                      grammar_id = 203?;
              }
              else if DC_EVChargeParameterType.EVEnergyRequest.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (EVEnergyRequest, PhysicalValueType); next=204
  
  
                      encode_ISO2PhysicalValueType(stream, &DC_EVChargeParameterType.EVEnergyRequest)?;
                      grammar_id = 204?;
              }
              else if DC_EVChargeParameterType.FullSOC.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (FullSOC, byte); next=205
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.FullSOC as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 205;
  
  
  
              }
              else if DC_EVChargeParameterType.BulkSOC.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (BulkSOC, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.BulkSOC as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 4)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           203=>{
               // Grammar: ID=203; read/write bits=3; START (EVEnergyRequest), START (FullSOC), START (BulkSOC), END Element
              if DC_EVChargeParameterType.EVEnergyRequest.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (EVEnergyRequest, PhysicalValueType); next=204
  
  
                      encode_ISO2PhysicalValueType(stream, &DC_EVChargeParameterType.EVEnergyRequest)?;
                      grammar_id = 204?;
              }
              else if DC_EVChargeParameterType.FullSOC.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (FullSOC, byte); next=205
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.FullSOC as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 205;
  
  
  
              }
              else if DC_EVChargeParameterType.BulkSOC.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (BulkSOC, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.BulkSOC as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           204=>{
               // Grammar: ID=204; read/write bits=2; START (FullSOC), START (BulkSOC), END Element
              if DC_EVChargeParameterType.FullSOC.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (FullSOC, byte); next=205
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.FullSOC as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 205;
  
  
  
              }
              else if DC_EVChargeParameterType.BulkSOC.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (BulkSOC, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.BulkSOC as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           205=>{
               // Grammar: ID=205; read/write bits=2; START (BulkSOC), END Element
              if DC_EVChargeParameterType.BulkSOC.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (BulkSOC, byte); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(7, DC_EVChargeParameterType.BulkSOC as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVChargeParameterType; base type=EVChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DepartureTime, unsignedInt (0, 1); EAmount, PhysicalValueType (1, 1); EVMaxVoltage, PhysicalValueType (1, 1); EVMaxCurrent, PhysicalValueType (1, 1); EVMinCurrent, PhysicalValueType (1, 1);
  pub fn encode_ISO2AC_EVChargeParameterType(stream: &mut ExiBitstream, AC_EVChargeParameterType: &ISO2AC_EVChargeParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 206;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           206=>{
               // Grammar: ID=206; read/write bits=2; START (DepartureTime), START (EAmount)
              if AC_EVChargeParameterType.DepartureTime.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (DepartureTime, unsignedLong); next=207
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_u32(AC_EVChargeParameterType.DepartureTime as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 207;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (EAmount, PhysicalValueType); next=208
  
  
                      encode_ISO2PhysicalValueType(stream, &AC_EVChargeParameterType.EAmount)?;
                      grammar_id = 208?;
              }
  }
           207=>{
               // Grammar: ID=207; read/write bits=1; START (EAmount)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=208
  
  
                  encode_ISO2PhysicalValueType(stream, &AC_EVChargeParameterType.EAmount)?;
                  grammar_id = 208?;
  }
           208=>{
               // Grammar: ID=208; read/write bits=1; START (EVMaxVoltage)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=209
  
  
                  encode_ISO2PhysicalValueType(stream, &AC_EVChargeParameterType.EVMaxVoltage)?;
                  grammar_id = 209?;
  }
           209=>{
               // Grammar: ID=209; read/write bits=1; START (EVMaxCurrent)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=210
  
  
                  encode_ISO2PhysicalValueType(stream, &AC_EVChargeParameterType.EVMaxCurrent)?;
                  grammar_id = 210?;
  }
           210=>{
               // Grammar: ID=210; read/write bits=1; START (EVMinCurrent)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=3
  
  
                  encode_ISO2PhysicalValueType(stream, &AC_EVChargeParameterType.EVMinCurrent)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVChargeParameterType; base type=; content type=ELEMENT-ONLY;
  //          abstract=True; final=False;
  // Particle: DepartureTime, unsignedInt (0, 1); DC_EVChargeParameter, DC_EVChargeParameterType (1, 1); AC_EVChargeParameter, AC_EVChargeParameterType (1, 1);
  pub fn encode_ISO2EVChargeParameterType(stream: &mut ExiBitstream, EVChargeParameterType: &ISO2EVChargeParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 211;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           211=>{
               // Grammar: ID=211; read/write bits=2; START (DepartureTime), START (DC_EVChargeParameter)
              if EVChargeParameterType.DepartureTime.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (DepartureTime, unsignedLong); next=212
  
  
                      stream.write_nbit_uint( 1, 0)?;
                      stream.write_u32(EVChargeParameterType.DepartureTime as u32)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 212;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (DC_EVChargeParameter, EVChargeParameterType); next=213
  
  
                      encode_ISO2DC_EVChargeParameterType(stream, &EVChargeParameterType.DC_EVChargeParameter)?;
                      grammar_id = 213?;
              }
  }
           212=>{
               // Grammar: ID=212; read/write bits=1; START (DC_EVChargeParameter)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVChargeParameterType); next=213
  
  
                  encode_ISO2DC_EVChargeParameterType(stream, &EVChargeParameterType.DC_EVChargeParameter)?;
                  grammar_id = 213?;
  }
           213=>{
               // Grammar: ID=213; read/write bits=1; START (AC_EVChargeParameter)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVChargeParameterType); next=3
  
  
                  encode_ISO2AC_EVChargeParameterType(stream, &EVChargeParameterType.AC_EVChargeParameter)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeService; type={urn:iso:15118:2:2013:MsgDataTypes}ChargeServiceType; base type=ServiceType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ServiceID, serviceIDType (1, 1); ServiceName, serviceNameType (0, 1); ServiceCategory, serviceCategoryType (1, 1); ServiceScope, serviceScopeType (0, 1); FreeService, boolean (1, 1); SupportedEnergyTransferMode, SupportedEnergyTransferModeType (1, 1);
  pub fn encode_ISO2ChargeServiceType(stream: &mut ExiBitstream, ChargeServiceType: &ISO2ChargeServiceType )->Result<(), BitstreamError> {
      let mut grammar_id = 214;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           214=>{
               // Grammar: ID=214; read/write bits=1; START (ServiceID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedShort); next=215
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(ChargeServiceType.ServiceID as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 215;
  
  
  }
           215=>{
               // Grammar: ID=215; read/write bits=2; START (ServiceName), START (ServiceCategory)
              if ChargeServiceType.ServiceName.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ServiceName, string); next=216
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((ChargeServiceType.ServiceName.len() as u16))?;
                      stream.write_characters(&ChargeServiceType.ServiceName.to_string(), ISO2ServiceName_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 216;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (ServiceCategory, string); next=217
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(2, ChargeServiceType.ServiceCategory)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 217;
              }
  }
           216=>{
               // Grammar: ID=216; read/write bits=1; START (ServiceCategory)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=217
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, ChargeServiceType.ServiceCategory)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 217;
  }
           217=>{
               // Grammar: ID=217; read/write bits=2; START (ServiceScope), START (FreeService)
              if ChargeServiceType.ServiceScope.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ServiceScope, string); next=218
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((ChargeServiceType.ServiceScope.len() as u16))?;
                      stream.write_characters(&ChargeServiceType.ServiceScope.to_string(), ISO2ServiceScope_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 218;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (FreeService, boolean); next=219
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ChargeServiceType.FreeService)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 219;
  
  
  
              }
  }
           218=>{
               // Grammar: ID=218; read/write bits=1; START (FreeService)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (boolean); next=219
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( ChargeServiceType.FreeService)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 219;
  
  
  }
           219=>{
               // Grammar: ID=219; read/write bits=1; START (SupportedEnergyTransferMode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (SupportedEnergyTransferModeType); next=3
  
  
                  encode_ISO2SupportedEnergyTransferModeType(stream, &ChargeServiceType.SupportedEnergyTransferMode)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SASchedules; type={urn:iso:15118:2:2013:MsgDataTypes}SASchedulesType; base type=; content type=empty;
  //          abstract=True; final=False;
  fn encode_ISO2SASchedulesType(stream: &mut ExiBitstream,SASchedulesType: &struct_type)->Result<(),BitstreamError>{
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
      let mut grammar_id = 220;
      let mut done = 0;
      let mut SAScheduleTuple_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           220=>{
               // Grammar: ID=220; read/write bits=1; START (SAScheduleTuple)
              if (SAScheduleTuple_currentIndex < SAScheduleListType.SAScheduleTuple.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (SAScheduleTupleType); next=221
  
  
                      encode_ISO2SAScheduleTupleType(stream, &SAScheduleListType.SAScheduleTuple.array[SAScheduleTuple_currentIndex])?;
                      SAScheduleTuple_currentIndex+=1;
                      grammar_id = 221;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           221=>{
               // Grammar: ID=221; read/write bits=2; START (SAScheduleTuple), END Element
              if (SAScheduleTuple_currentIndex < SAScheduleListType.SAScheduleTuple.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SAScheduleTupleType); next=222
  
  
                      encode_ISO2SAScheduleTupleType(stream, &SAScheduleListType.SAScheduleTuple.array[SAScheduleTuple_currentIndex])?;
                      SAScheduleTuple_currentIndex+=1;
                      grammar_id = 222;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           222=>{
               // Grammar: ID=222; read/write bits=2; START (SAScheduleTuple), END Element
              if (SAScheduleTuple_currentIndex < SAScheduleListType.SAScheduleTuple.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (SAScheduleTupleType); next=3
  
  
                      encode_ISO2SAScheduleTupleType(stream, &SAScheduleListType.SAScheduleTuple.array[SAScheduleTuple_currentIndex])?;
                      SAScheduleTuple_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingProfile; type={urn:iso:15118:2:2013:MsgDataTypes}ChargingProfileType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ProfileEntry, ProfileEntryType (1, 24);
  pub fn encode_ISO2ChargingProfileType(stream: &mut ExiBitstream, ChargingProfileType: &ISO2ChargingProfileType )->Result<(), BitstreamError> {
      let mut grammar_id = 223;
      let mut done = 0;
      let mut ProfileEntry_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           223=>{
               // Grammar: ID=223; read/write bits=1; START (ProfileEntry)
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (ProfileEntryType); next=224
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
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
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=225
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 225;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           225=>{
               // Grammar: ID=225; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=226
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 226;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           226=>{
               // Grammar: ID=226; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=227
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 227;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           227=>{
               // Grammar: ID=227; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=228
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 228;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           228=>{
               // Grammar: ID=228; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=229
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 229;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           229=>{
               // Grammar: ID=229; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=230
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 230;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           230=>{
               // Grammar: ID=230; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=231
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 231;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           231=>{
               // Grammar: ID=231; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=232
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 232;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           232=>{
               // Grammar: ID=232; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=233
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 233;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           233=>{
               // Grammar: ID=233; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=234
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 234;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           234=>{
               // Grammar: ID=234; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=235
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 235;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           235=>{
               // Grammar: ID=235; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=236
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 236;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           236=>{
               // Grammar: ID=236; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=237
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 237;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           237=>{
               // Grammar: ID=237; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=238
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 238;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           238=>{
               // Grammar: ID=238; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=239
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 239;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           239=>{
               // Grammar: ID=239; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=240
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 240;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           240=>{
               // Grammar: ID=240; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=241
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 241;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           241=>{
               // Grammar: ID=241; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=242
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 242;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           242=>{
               // Grammar: ID=242; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=243
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 243;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           243=>{
               // Grammar: ID=243; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=244
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 244;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           244=>{
               // Grammar: ID=244; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=245
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 245;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           245=>{
               // Grammar: ID=245; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=246
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 246;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           246=>{
               // Grammar: ID=246; read/write bits=2; START (ProfileEntry), END Element
              if (ProfileEntry_currentIndex < ChargingProfileType.ProfileEntry.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ProfileEntryType); next=3
  
  
                      encode_ISO2ProfileEntryType(stream, &ChargingProfileType.ProfileEntry.array[ProfileEntry_currentIndex])?;
                      ProfileEntry_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ContractSignatureEncryptedPrivateKey; type={urn:iso:15118:2:2013:MsgDataTypes}ContractSignatureEncryptedPrivateKeyType; base type=privateKeyType; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); CONTENT, ContractSignatureEncryptedPrivateKeyType (1, 1);
  pub fn encode_ISO2ContractSignatureEncryptedPrivateKeyType(stream: &mut ExiBitstream, ContractSignatureEncryptedPrivateKeyType: &ISO2ContractSignatureEncryptedPrivateKeyType )->Result<(), BitstreamError> {
      let mut grammar_id = 247;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           247=>{
               // Grammar: ID=247; read/write bits=1; START (Id)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (NCName); next=248
  
              // string should not be found in table, so add 2
               stream.write_u16((ContractSignatureEncryptedPrivateKeyType.Id.len() as u16))?;
              stream.write_characters(&ContractSignatureEncryptedPrivateKeyType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
              grammar_id = 248;
              }
              }
  }
           248=>{
               // Grammar: ID=248; read/write bits=1; START (CONTENT)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (base64Binary); next=3
                  stream.write_u16(ContractSignatureEncryptedPrivateKeyType.CONTENT.len() as u16)?;
                  stream.write_bytes(&ContractSignatureEncryptedPrivateKeyType.CONTENT)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceList; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Service, ServiceType (1, 8);
  pub fn encode_ISO2ServiceListType(stream: &mut ExiBitstream, ServiceListType: &ISO2ServiceListType )->Result<(), BitstreamError> {
      let mut grammar_id = 249;
      let mut done = 0;
      let mut Service_currentIndex: u16 = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           249=>{
               // Grammar: ID=249; read/write bits=1; START (Service)
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(1,0)?;
                  // Event: START (ServiceType); next=250
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service.array[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 250;
  
              }
              else
              {
                  return Err(BitstreamError::UnknownGrammarId);
              }
  }
           250=>{
               // Grammar: ID=250; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ServiceType); next=251
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service.array[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 251;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           251=>{
               // Grammar: ID=251; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ServiceType); next=252
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service.array[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 252;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           252=>{
               // Grammar: ID=252; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ServiceType); next=253
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service.array[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 253;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           253=>{
               // Grammar: ID=253; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ServiceType); next=254
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service.array[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 254;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           254=>{
               // Grammar: ID=254; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ServiceType); next=255
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service.array[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 255;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           255=>{
               // Grammar: ID=255; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ServiceType); next=256
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service.array[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 256;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           256=>{
               // Grammar: ID=256; read/write bits=2; START (Service), END Element
              if (Service_currentIndex < ServiceListType.Service.arrayLen)
              {
                  stream.write_nbit_uint(2,0)?;
                  // Event: START (ServiceType); next=3
  
  
                      encode_ISO2ServiceType(stream, &ServiceListType.Service.array[Service_currentIndex])?;
                      Service_currentIndex+=1;
                      grammar_id = 3;
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVSEChargeParameterType; base type=; content type=empty;
  //          abstract=True; final=False;
  fn encode_ISO2EVSEChargeParameterType(stream: &mut ExiBitstream,EVSEChargeParameterType: &struct_type)->Result<(),BitstreamError>{
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
      let mut grammar_id = 257;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           257=>{
               // Grammar: ID=257; read/write bits=1; START (AC_EVSEStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVSEStatusType); next=258
  
  
                  encode_ISO2AC_EVSEStatusType(stream, &AC_EVSEChargeParameterType.AC_EVSEStatus)?;
                  grammar_id = 258?;
  }
           258=>{
               // Grammar: ID=258; read/write bits=1; START (EVSENominalVoltage)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=259
  
  
                  encode_ISO2PhysicalValueType(stream, &AC_EVSEChargeParameterType.EVSENominalVoltage)?;
                  grammar_id = 259?;
  }
           259=>{
               // Grammar: ID=259; read/write bits=1; START (EVSEMaxCurrent)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=3
  
  
                  encode_ISO2PhysicalValueType(stream, &AC_EVSEChargeParameterType.EVSEMaxCurrent)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEChargeParameterType; base type=EVSEChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEMaximumCurrentLimit, PhysicalValueType (1, 1); EVSEMaximumPowerLimit, PhysicalValueType (1, 1); EVSEMaximumVoltageLimit, PhysicalValueType (1, 1); EVSEMinimumCurrentLimit, PhysicalValueType (1, 1); EVSEMinimumVoltageLimit, PhysicalValueType (1, 1); EVSECurrentRegulationTolerance, PhysicalValueType (0, 1); EVSEPeakCurrentRipple, PhysicalValueType (1, 1); EVSEEnergyToBeDelivered, PhysicalValueType (0, 1);
  pub fn encode_ISO2DC_EVSEChargeParameterType(stream: &mut ExiBitstream, DC_EVSEChargeParameterType: &ISO2DC_EVSEChargeParameterType )->Result<(), BitstreamError> {
      let mut grammar_id = 260;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           260=>{
               // Grammar: ID=260; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVSEStatusType); next=261
  
  
                  encode_ISO2DC_EVSEStatusType(stream, &DC_EVSEChargeParameterType.DC_EVSEStatus)?;
                  grammar_id = 261?;
  }
           261=>{
               // Grammar: ID=261; read/write bits=1; START (EVSEMaximumCurrentLimit)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=262
  
  
                  encode_ISO2PhysicalValueType(stream, &DC_EVSEChargeParameterType.EVSEMaximumCurrentLimit)?;
                  grammar_id = 262?;
  }
           262=>{
               // Grammar: ID=262; read/write bits=1; START (EVSEMaximumPowerLimit)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=263
  
  
                  encode_ISO2PhysicalValueType(stream, &DC_EVSEChargeParameterType.EVSEMaximumPowerLimit)?;
                  grammar_id = 263?;
  }
           263=>{
               // Grammar: ID=263; read/write bits=1; START (EVSEMaximumVoltageLimit)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=264
  
  
                  encode_ISO2PhysicalValueType(stream, &DC_EVSEChargeParameterType.EVSEMaximumVoltageLimit)?;
                  grammar_id = 264?;
  }
           264=>{
               // Grammar: ID=264; read/write bits=1; START (EVSEMinimumCurrentLimit)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=265
  
  
                  encode_ISO2PhysicalValueType(stream, &DC_EVSEChargeParameterType.EVSEMinimumCurrentLimit)?;
                  grammar_id = 265?;
  }
           265=>{
               // Grammar: ID=265; read/write bits=1; START (EVSEMinimumVoltageLimit)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=266
  
  
                  encode_ISO2PhysicalValueType(stream, &DC_EVSEChargeParameterType.EVSEMinimumVoltageLimit)?;
                  grammar_id = 266?;
  }
           266=>{
               // Grammar: ID=266; read/write bits=2; START (EVSECurrentRegulationTolerance), START (EVSEPeakCurrentRipple)
              if DC_EVSEChargeParameterType.EVSECurrentRegulationTolerance.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (EVSECurrentRegulationTolerance, PhysicalValueType); next=267
  
  
                      encode_ISO2PhysicalValueType(stream, &DC_EVSEChargeParameterType.EVSECurrentRegulationTolerance)?;
                      grammar_id = 267?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (EVSEPeakCurrentRipple, PhysicalValueType); next=268
  
  
                      encode_ISO2PhysicalValueType(stream, &DC_EVSEChargeParameterType.EVSEPeakCurrentRipple)?;
                      grammar_id = 268?;
              }
  }
           267=>{
               // Grammar: ID=267; read/write bits=1; START (EVSEPeakCurrentRipple)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=268
  
  
                  encode_ISO2PhysicalValueType(stream, &DC_EVSEChargeParameterType.EVSEPeakCurrentRipple)?;
                  grammar_id = 268?;
  }
           268=>{
               // Grammar: ID=268; read/write bits=2; START (EVSEEnergyToBeDelivered), END Element
              if DC_EVSEChargeParameterType.EVSEEnergyToBeDelivered.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (EVSEEnergyToBeDelivered, PhysicalValueType); next=3
  
  
                      encode_ISO2PhysicalValueType(stream, &DC_EVSEChargeParameterType.EVSEEnergyToBeDelivered)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVPowerDeliveryParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVPowerDeliveryParameterType; base type=; content type=empty;
  //          abstract=True; final=False;
  fn encode_ISO2EVPowerDeliveryParameterType(stream: &mut ExiBitstream,EVPowerDeliveryParameterType: &struct_type)->Result<(),BitstreamError>{
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
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           269=>{
               // Grammar: ID=269; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVStatusType); next=270
  
  
                  encode_ISO2DC_EVStatusType(stream, &DC_EVPowerDeliveryParameterType.DC_EVStatus)?;
                  grammar_id = 270?;
  }
           270=>{
               // Grammar: ID=270; read/write bits=2; START (BulkChargingComplete), START (ChargingComplete)
              if DC_EVPowerDeliveryParameterType.BulkChargingComplete.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (BulkChargingComplete, boolean); next=271
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( DC_EVPowerDeliveryParameterType.BulkChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 271;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
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
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (boolean); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( DC_EVPowerDeliveryParameterType.ChargingComplete)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeterInfo; type={urn:iso:15118:2:2013:MsgDataTypes}MeterInfoType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: MeterID, meterIDType (1, 1); MeterReading, unsignedLong (0, 1); SigMeterReading, sigMeterReadingType (0, 1); MeterStatus, meterStatusType (0, 1); TMeter, long (0, 1);
  pub fn encode_ISO2MeterInfoType(stream: &mut ExiBitstream, MeterInfoType: &ISO2MeterInfoType )->Result<(), BitstreamError> {
      let mut grammar_id = 272;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           272=>{
               // Grammar: ID=272; read/write bits=1; START (MeterID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=273
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
                   stream.write_u16((MeterInfoType.MeterID.len() as u16))?;
                  stream.write_characters(&MeterInfoType.MeterID.to_string(), ISO2MeterID_CHARACTER_SIZE)?;
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 273;
  }
           273=>{
               // Grammar: ID=273; read/write bits=3; START (MeterReading), START (SigMeterReading), START (MeterStatus), START (TMeter), END Element
              if MeterInfoType.MeterReading.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (MeterReading, nonNegativeInteger); next=274
  
  
                       stream.write_nbit_uint(1, 0)?;
                       stream.write_u64(MeterInfoType.MeterReading as u64)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 274;
  
  
  
              }
              else if MeterInfoType.SigMeterReading.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (SigMeterReading, base64Binary); next=275
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(MeterInfoType.SigMeterReading.len() as u16)?;
                              stream.write_bytes( &MeterInfoType.SigMeterReading)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 275;
  
  
  
              }
              else if MeterInfoType.MeterStatus.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (MeterStatus, short); next=276
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( MeterInfoType.MeterStatus as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 276;
  
  
  
  
              }
              else if MeterInfoType.TMeter.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (TMeter, integer); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i64(MeterInfoType.TMeter)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 4)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           274=>{
               // Grammar: ID=274; read/write bits=3; START (SigMeterReading), START (MeterStatus), START (TMeter), END Element
              if MeterInfoType.SigMeterReading.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (SigMeterReading, base64Binary); next=275
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(MeterInfoType.SigMeterReading.len() as u16)?;
                              stream.write_bytes( &MeterInfoType.SigMeterReading)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 275;
  
  
  
              }
              else if MeterInfoType.MeterStatus.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (MeterStatus, short); next=276
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( MeterInfoType.MeterStatus as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 276;
  
  
  
  
              }
              else if MeterInfoType.TMeter.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (TMeter, integer); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i64(MeterInfoType.TMeter)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           275=>{
               // Grammar: ID=275; read/write bits=2; START (MeterStatus), START (TMeter), END Element
              if MeterInfoType.MeterStatus.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (MeterStatus, short); next=276
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( MeterInfoType.MeterStatus as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 276;
  
  
  
  
              }
              else if MeterInfoType.TMeter.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (TMeter, integer); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i64(MeterInfoType.TMeter)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           276=>{
               // Grammar: ID=276; read/write bits=2; START (TMeter), END Element
              if MeterInfoType.TMeter.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (TMeter, integer); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i64(MeterInfoType.TMeter)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}DHpublickey; type={urn:iso:15118:2:2013:MsgDataTypes}DiffieHellmanPublickeyType; base type=dHpublickeyType; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); CONTENT, DiffieHellmanPublickeyType (1, 1);
  pub fn encode_ISO2DiffieHellmanPublickeyType(stream: &mut ExiBitstream, DiffieHellmanPublickeyType: &ISO2DiffieHellmanPublickeyType )->Result<(), BitstreamError> {
      let mut grammar_id = 277;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           277=>{
               // Grammar: ID=277; read/write bits=1; START (Id)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (NCName); next=278
  
              // string should not be found in table, so add 2
               stream.write_u16((DiffieHellmanPublickeyType.Id.len() as u16))?;
              stream.write_characters(&DiffieHellmanPublickeyType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
              grammar_id = 278;
              }
              }
  }
           278=>{
               // Grammar: ID=278; read/write bits=1; START (CONTENT)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (base64Binary); next=3
                  stream.write_u16(DiffieHellmanPublickeyType.CONTENT.len() as u16)?;
                  stream.write_bytes(&DiffieHellmanPublickeyType.CONTENT)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}eMAID; type={urn:iso:15118:2:2013:MsgDataTypes}EMAIDType; base type=eMAIDType; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); CONTENT, EMAIDType (1, 1);
  pub fn encode_ISO2EMAIDType(stream: &mut ExiBitstream, EMAIDType: &ISO2EMAIDType )->Result<(), BitstreamError> {
      let mut grammar_id = 279;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           279=>{
               // Grammar: ID=279; read/write bits=1; START (Id)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (NCName); next=280
  
              // string should not be found in table, so add 2
               stream.write_u16((EMAIDType.Id.len() as u16))?;
              stream.write_characters(&EMAIDType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
              grammar_id = 280;
              }
              }
  }
           280=>{
               // Grammar: ID=280; read/write bits=1; START (CONTENT)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=3
  
              // string should not be found in table, so add 2
               stream.write_u16((EMAIDType.CONTENT.len() as u16))?;
              stream.write_characters(&EMAIDType.CONTENT.to_string(), ISO2CONTENT_CHARACTER_SIZE)?;
              grammar_id = 3;
              }
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}Header; type={urn:iso:15118:2:2013:MsgHeader}MessageHeaderType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SessionID, sessionIDType (1, 1); Notification, NotificationType (0, 1); Signature, SignatureType (0, 1);
  pub fn encode_ISO2MessageHeaderType(stream: &mut ExiBitstream, MessageHeaderType: &ISO2MessageHeaderType )->Result<(), BitstreamError> {
      let mut grammar_id = 281;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           281=>{
               // Grammar: ID=281; read/write bits=1; START (SessionID)
              stream.write_nbit_uint( 1, 0)?;
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
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Notification, NotificationType); next=283
  
  
                      encode_ISO2NotificationType(stream, &MessageHeaderType.Notification)?;
                      grammar_id = 283?;
              }
              else if MessageHeaderType.Signature.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (Signature, SignatureType); next=3
  
  
                      encode_ISO2SignatureType(stream, &MessageHeaderType.Signature)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           283=>{
               // Grammar: ID=283; read/write bits=2; START (Signature), END Element
              if MessageHeaderType.Signature.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Signature, SignatureType); next=3
  
  
                      encode_ISO2SignatureType(stream, &MessageHeaderType.Signature)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDetailRes; type={urn:iso:15118:2:2013:MsgBody}ServiceDetailResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); ServiceID, serviceIDType (1, 1); ServiceParameterList, ServiceParameterListType (0, 1);
  pub fn encode_ISO2ServiceDetailResType(stream: &mut ExiBitstream, ServiceDetailResType: &ISO2ServiceDetailResType )->Result<(), BitstreamError> {
      let mut grammar_id = 284;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           284=>{
               // Grammar: ID=284; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=285
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, ServiceDetailResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 285;
  }
           285=>{
               // Grammar: ID=285; read/write bits=1; START (ServiceID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedShort); next=286
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(ServiceDetailResType.ServiceID as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 286;
  
  
  }
           286=>{
               // Grammar: ID=286; read/write bits=2; START (ServiceParameterList), END Element
              if ServiceDetailResType.ServiceParameterList.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ServiceParameterList, ServiceParameterListType); next=3
  
  
                      encode_ISO2ServiceParameterListType(stream, &ServiceDetailResType.ServiceParameterList)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PreChargeReq; type={urn:iso:15118:2:2013:MsgBody}PreChargeReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1); EVTargetVoltage, PhysicalValueType (1, 1); EVTargetCurrent, PhysicalValueType (1, 1);
  pub fn encode_ISO2PreChargeReqType(stream: &mut ExiBitstream, PreChargeReqType: &ISO2PreChargeReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 287;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           287=>{
               // Grammar: ID=287; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVStatusType); next=288
  
  
                  encode_ISO2DC_EVStatusType(stream, &PreChargeReqType.DC_EVStatus)?;
                  grammar_id = 288?;
  }
           288=>{
               // Grammar: ID=288; read/write bits=1; START (EVTargetVoltage)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=289
  
  
                  encode_ISO2PhysicalValueType(stream, &PreChargeReqType.EVTargetVoltage)?;
                  grammar_id = 289?;
  }
           289=>{
               // Grammar: ID=289; read/write bits=1; START (EVTargetCurrent)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=3
  
  
                  encode_ISO2PhysicalValueType(stream, &PreChargeReqType.EVTargetCurrent)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CableCheckRes; type={urn:iso:15118:2:2013:MsgBody}CableCheckResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1);
  pub fn encode_ISO2CableCheckResType(stream: &mut ExiBitstream, CableCheckResType: &ISO2CableCheckResType )->Result<(), BitstreamError> {
      let mut grammar_id = 290;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           290=>{
               // Grammar: ID=290; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=291
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, CableCheckResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 291;
  }
           291=>{
               // Grammar: ID=291; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVSEStatusType); next=292
  
  
                  encode_ISO2DC_EVSEStatusType(stream, &CableCheckResType.DC_EVSEStatus)?;
                  grammar_id = 292?;
  }
           292=>{
               // Grammar: ID=292; read/write bits=1; START (EVSEProcessing)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, CableCheckResType.EVSEProcessing)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PowerDeliveryRes; type={urn:iso:15118:2:2013:MsgBody}PowerDeliveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (0, 1); DC_EVSEStatus, DC_EVSEStatusType (0, 1); EVSEStatus, EVSEStatusType (0, 1);
  pub fn encode_ISO2PowerDeliveryResType(stream: &mut ExiBitstream, PowerDeliveryResType: &ISO2PowerDeliveryResType )->Result<(), BitstreamError> {
      let mut grammar_id = 293;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           293=>{
               // Grammar: ID=293; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=294
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, PowerDeliveryResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 294;
  }
           294=>{
               // Grammar: ID=294; read/write bits=2; START (AC_EVSEStatus), START (DC_EVSEStatus), START (EVSEStatus)
              if PowerDeliveryResType.AC_EVSEStatus.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (AC_EVSEStatus, EVSEStatusType); next=3
  
  
                      encode_ISO2AC_EVSEStatusType(stream, &PowerDeliveryResType.AC_EVSEStatus)?;
                      grammar_id = 3?;
              }
              else if PowerDeliveryResType.DC_EVSEStatus.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (DC_EVSEStatus, EVSEStatusType); next=3
  
  
                      encode_ISO2DC_EVSEStatusType(stream, &PowerDeliveryResType.DC_EVSEStatus)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (EVSEStatus, EVSEStatusType); next=3
  
  
                      encode_ISO2EVSEStatusType(stream, &PowerDeliveryResType.EVSEStatus)?;
                      grammar_id = 3?;
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionSetupRes; type={urn:iso:15118:2:2013:MsgBody}SessionSetupResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEID, evseIDType (1, 1); EVSETimeStamp, long (0, 1);
  pub fn encode_ISO2SessionSetupResType(stream: &mut ExiBitstream, SessionSetupResType: &ISO2SessionSetupResType )->Result<(), BitstreamError> {
      let mut grammar_id = 295;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           295=>{
               // Grammar: ID=295; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=296
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, SessionSetupResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 296;
  }
           296=>{
               // Grammar: ID=296; read/write bits=1; START (EVSEID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=297
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
                   stream.write_u16((SessionSetupResType.EVSEID.len() as u16))?;
                  stream.write_characters(&SessionSetupResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 297;
  }
           297=>{
               // Grammar: ID=297; read/write bits=2; START (EVSETimeStamp), END Element
              if SessionSetupResType.EVSETimeStamp.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (EVSETimeStamp, integer); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i64(SessionSetupResType.EVSETimeStamp)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CableCheckReq; type={urn:iso:15118:2:2013:MsgBody}CableCheckReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1);
  pub fn encode_ISO2CableCheckReqType(stream: &mut ExiBitstream, CableCheckReqType: &ISO2CableCheckReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 298;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           298=>{
               // Grammar: ID=298; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVStatusType); next=3
  
  
                  encode_ISO2DC_EVStatusType(stream, &CableCheckReqType.DC_EVStatus)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionStopReq; type={urn:iso:15118:2:2013:MsgBody}SessionStopReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ChargingSession, chargingSessionType (1, 1);
  pub fn encode_ISO2SessionStopReqType(stream: &mut ExiBitstream, SessionStopReqType: &ISO2SessionStopReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 299;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           299=>{
               // Grammar: ID=299; read/write bits=1; START (ChargingSession)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(1, SessionStopReqType.ChargingSession)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingStatusRes; type={urn:iso:15118:2:2013:MsgBody}ChargingStatusResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEID, evseIDType (1, 1); SAScheduleTupleID, SAIDType (1, 1); EVSEMaxCurrent, PhysicalValueType (0, 1); MeterInfo, MeterInfoType (0, 1); ReceiptRequired, boolean (0, 1); AC_EVSEStatus, AC_EVSEStatusType (1, 1);
  pub fn encode_ISO2ChargingStatusResType(stream: &mut ExiBitstream, ChargingStatusResType: &ISO2ChargingStatusResType )->Result<(), BitstreamError> {
      let mut grammar_id = 300;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           300=>{
               // Grammar: ID=300; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=301
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, ChargingStatusResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 301;
  }
           301=>{
               // Grammar: ID=301; read/write bits=1; START (EVSEID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=302
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
                   stream.write_u16((ChargingStatusResType.EVSEID.len() as u16))?;
                  stream.write_characters(&ChargingStatusResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 302;
  }
           302=>{
               // Grammar: ID=302; read/write bits=1; START (SAScheduleTupleID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedByte); next=303
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(8, (ChargingStatusResType.SAScheduleTupleID as u32 - 1))?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 303;
  
  
  }
           303=>{
               // Grammar: ID=303; read/write bits=3; START (EVSEMaxCurrent), START (MeterInfo), START (ReceiptRequired), START (AC_EVSEStatus)
              if ChargingStatusResType.EVSEMaxCurrent.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (EVSEMaxCurrent, PhysicalValueType); next=304
  
  
                      encode_ISO2PhysicalValueType(stream, &ChargingStatusResType.EVSEMaxCurrent)?;
                      grammar_id = 304?;
              }
              else if ChargingStatusResType.MeterInfo.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (MeterInfo, MeterInfoType); next=305
  
  
                      encode_ISO2MeterInfoType(stream, &ChargingStatusResType.MeterInfo)?;
                      grammar_id = 305?;
              }
              else if ChargingStatusResType.ReceiptRequired.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (ReceiptRequired, boolean); next=306
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ChargingStatusResType.ReceiptRequired)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 306;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (AC_EVSEStatus, EVSEStatusType); next=3
  
  
                      encode_ISO2AC_EVSEStatusType(stream, &ChargingStatusResType.AC_EVSEStatus)?;
                      grammar_id = 3?;
              }
  }
           304=>{
               // Grammar: ID=304; read/write bits=2; START (MeterInfo), START (ReceiptRequired), START (AC_EVSEStatus)
              if ChargingStatusResType.MeterInfo.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (MeterInfo, MeterInfoType); next=305
  
  
                      encode_ISO2MeterInfoType(stream, &ChargingStatusResType.MeterInfo)?;
                      grammar_id = 305?;
              }
              else if ChargingStatusResType.ReceiptRequired.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (ReceiptRequired, boolean); next=306
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ChargingStatusResType.ReceiptRequired)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 306;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (AC_EVSEStatus, EVSEStatusType); next=3
  
  
                      encode_ISO2AC_EVSEStatusType(stream, &ChargingStatusResType.AC_EVSEStatus)?;
                      grammar_id = 3?;
              }
  }
           305=>{
               // Grammar: ID=305; read/write bits=2; START (ReceiptRequired), START (AC_EVSEStatus)
              if ChargingStatusResType.ReceiptRequired.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ReceiptRequired, boolean); next=306
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( ChargingStatusResType.ReceiptRequired)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 306;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (AC_EVSEStatus, EVSEStatusType); next=3
  
  
                      encode_ISO2AC_EVSEStatusType(stream, &ChargingStatusResType.AC_EVSEStatus)?;
                      grammar_id = 3?;
              }
  }
           306=>{
               // Grammar: ID=306; read/write bits=1; START (AC_EVSEStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVSEStatusType); next=3
  
  
                  encode_ISO2AC_EVSEStatusType(stream, &ChargingStatusResType.AC_EVSEStatus)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryReq; type={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: MaxEntriesSAScheduleTuple, unsignedShort (0, 1); RequestedEnergyTransferMode, EnergyTransferModeType (1, 1); AC_EVChargeParameter, AC_EVChargeParameterType (0, 1); DC_EVChargeParameter, DC_EVChargeParameterType (0, 1); EVChargeParameter, EVChargeParameterType (0, 1);
  pub fn encode_ISO2ChargeParameterDiscoveryReqType(stream: &mut ExiBitstream, ChargeParameterDiscoveryReqType: &ISO2ChargeParameterDiscoveryReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 307;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           307=>{
               // Grammar: ID=307; read/write bits=2; START (MaxEntriesSAScheduleTuple), START (RequestedEnergyTransferMode)
              if ChargeParameterDiscoveryReqType.MaxEntriesSAScheduleTuple.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (MaxEntriesSAScheduleTuple, unsignedInt); next=308
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(ChargeParameterDiscoveryReqType.MaxEntriesSAScheduleTuple as u16)?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 308;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (RequestedEnergyTransferMode, string); next=309
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(3, ChargeParameterDiscoveryReqType.RequestedEnergyTransferMode)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 309;
              }
  }
           308=>{
               // Grammar: ID=308; read/write bits=1; START (RequestedEnergyTransferMode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=309
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(3, ChargeParameterDiscoveryReqType.RequestedEnergyTransferMode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 309;
  }
           309=>{
               // Grammar: ID=309; read/write bits=2; START (AC_EVChargeParameter), START (DC_EVChargeParameter), START (EVChargeParameter)
              if ChargeParameterDiscoveryReqType.AC_EVChargeParameter.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (AC_EVChargeParameter, EVChargeParameterType); next=3
  
  
                      encode_ISO2AC_EVChargeParameterType(stream, &ChargeParameterDiscoveryReqType.AC_EVChargeParameter)?;
                      grammar_id = 3?;
              }
              else if ChargeParameterDiscoveryReqType.DC_EVChargeParameter.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (DC_EVChargeParameter, EVChargeParameterType); next=3
  
  
                      encode_ISO2DC_EVChargeParameterType(stream, &ChargeParameterDiscoveryReqType.DC_EVChargeParameter)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (EVChargeParameter, EVChargeParameterType); next=3
  
  
                      encode_ISO2EVChargeParameterType(stream, &ChargeParameterDiscoveryReqType.EVChargeParameter)?;
                      grammar_id = 3?;
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}AuthorizationReq; type={urn:iso:15118:2:2013:MsgBody}AuthorizationReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (0, 1); GenChallenge, genChallengeType (0, 1);
  pub fn encode_ISO2AuthorizationReqType(stream: &mut ExiBitstream, AuthorizationReqType: &ISO2AuthorizationReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 310;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           310=>{
               // Grammar: ID=310; read/write bits=2; START (Id), START (GenChallenge), END Element
              if AuthorizationReqType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Id, NCName); next=311
  
                  // string should not be found in table, so add 2
                   stream.write_u16((AuthorizationReqType.Id.len() as u16))?;
                  stream.write_characters(&AuthorizationReqType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 311;
                  }
                  }
              }
              else if AuthorizationReqType.GenChallenge.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (GenChallenge, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(AuthorizationReqType.GenChallenge.len() as u16)?;
                              stream.write_bytes( &AuthorizationReqType.GenChallenge)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           311=>{
               // Grammar: ID=311; read/write bits=2; START (GenChallenge), END Element
              if AuthorizationReqType.GenChallenge.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (GenChallenge, base64Binary); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16(AuthorizationReqType.GenChallenge.len() as u16)?;
                              stream.write_bytes( &AuthorizationReqType.GenChallenge)?;
                                  // encode END Element
                                  stream.write_nbit_uint(1, 0)?;
                                  grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateInstallationRes; type={urn:iso:15118:2:2013:MsgBody}CertificateInstallationResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); SAProvisioningCertificateChain, CertificateChainType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (1, 1); DHpublickey, DiffieHellmanPublickeyType (1, 1); eMAID, EMAIDType (1, 1);
  pub fn encode_ISO2CertificateInstallationResType(stream: &mut ExiBitstream, CertificateInstallationResType: &ISO2CertificateInstallationResType )->Result<(), BitstreamError> {
      let mut grammar_id = 312;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           312=>{
               // Grammar: ID=312; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=313
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, CertificateInstallationResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 313;
  }
           313=>{
               // Grammar: ID=313; read/write bits=1; START (SAProvisioningCertificateChain)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (CertificateChainType); next=314
  
  
                  encode_ISO2CertificateChainType(stream, &CertificateInstallationResType.SAProvisioningCertificateChain)?;
                  grammar_id = 314?;
  }
           314=>{
               // Grammar: ID=314; read/write bits=1; START (ContractSignatureCertChain)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (CertificateChainType); next=315
  
  
                  encode_ISO2CertificateChainType(stream, &CertificateInstallationResType.ContractSignatureCertChain)?;
                  grammar_id = 315?;
  }
           315=>{
               // Grammar: ID=315; read/write bits=1; START (ContractSignatureEncryptedPrivateKey)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (privateKeyType); next=316
  
  
                  encode_ISO2ContractSignatureEncryptedPrivateKeyType(stream, &CertificateInstallationResType.ContractSignatureEncryptedPrivateKey)?;
                  grammar_id = 316?;
  }
           316=>{
               // Grammar: ID=316; read/write bits=1; START (DHpublickey)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (dHpublickeyType); next=317
  
  
                  encode_ISO2DiffieHellmanPublickeyType(stream, &CertificateInstallationResType.DHpublickey)?;
                  grammar_id = 317?;
  }
           317=>{
               // Grammar: ID=317; read/write bits=1; START (eMAID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (eMAIDType); next=3
  
  
                  encode_ISO2EMAIDType(stream, &CertificateInstallationResType.eMAID)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionReq; type={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: SelectedPaymentOption, paymentOptionType (1, 1); SelectedServiceList, SelectedServiceListType (1, 1);
  pub fn encode_ISO2PaymentServiceSelectionReqType(stream: &mut ExiBitstream, PaymentServiceSelectionReqType: &ISO2PaymentServiceSelectionReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 318;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           318=>{
               // Grammar: ID=318; read/write bits=1; START (SelectedPaymentOption)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=319
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(1, PaymentServiceSelectionReqType.SelectedPaymentOption)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 319;
  }
           319=>{
               // Grammar: ID=319; read/write bits=1; START (SelectedServiceList)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (SelectedServiceListType); next=3
  
  
                  encode_ISO2SelectedServiceListType(stream, &PaymentServiceSelectionReqType.SelectedServiceList)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingStatusReq; type={urn:iso:15118:2:2013:MsgBody}ChargingStatusReqType; base type=BodyBaseType; content type=empty;
  //          abstract=False; final=False; derivation=extension;
  fn encode_ISO2ChargingStatusReqType(stream: &mut ExiBitstream,ChargingStatusReqType: &struct_type)->Result<(),BitstreamError>{
      // Element has no particles, so the function just encodes END Element
      let _ = ChargingStatusReqType;// To silence the unused variable warning
  
  //    error:i32= exi_basetypes_encoder_nbit_uint(stream, 1, 0);
      stream.write_nbit_uint(1,0)?;
       Ok(())
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryRes; type={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); PaymentOptionList, PaymentOptionListType (1, 1); ChargeService, ChargeServiceType (1, 1); ServiceList, ServiceListType (0, 1);
  pub fn encode_ISO2ServiceDiscoveryResType(stream: &mut ExiBitstream, ServiceDiscoveryResType: &ISO2ServiceDiscoveryResType )->Result<(), BitstreamError> {
      let mut grammar_id = 320;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           320=>{
               // Grammar: ID=320; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=321
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, ServiceDiscoveryResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 321;
  }
           321=>{
               // Grammar: ID=321; read/write bits=1; START (PaymentOptionList)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PaymentOptionListType); next=322
  
  
                  encode_ISO2PaymentOptionListType(stream, &ServiceDiscoveryResType.PaymentOptionList)?;
                  grammar_id = 322?;
  }
           322=>{
               // Grammar: ID=322; read/write bits=1; START (ChargeService)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (ServiceType); next=323
  
  
                  encode_ISO2ChargeServiceType(stream, &ServiceDiscoveryResType.ChargeService)?;
                  grammar_id = 323?;
  }
           323=>{
               // Grammar: ID=323; read/write bits=2; START (ServiceList), END Element
              if ServiceDiscoveryResType.ServiceList.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ServiceList, ServiceListType); next=3
  
  
                      encode_ISO2ServiceListType(stream, &ServiceDiscoveryResType.ServiceList)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateUpdateRes; type={urn:iso:15118:2:2013:MsgBody}CertificateUpdateResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); SAProvisioningCertificateChain, CertificateChainType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (1, 1); DHpublickey, DiffieHellmanPublickeyType (1, 1); eMAID, EMAIDType (1, 1); RetryCounter, short (0, 1);
  pub fn encode_ISO2CertificateUpdateResType(stream: &mut ExiBitstream, CertificateUpdateResType: &ISO2CertificateUpdateResType )->Result<(), BitstreamError> {
      let mut grammar_id = 324;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           324=>{
               // Grammar: ID=324; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=325
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, CertificateUpdateResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 325;
  }
           325=>{
               // Grammar: ID=325; read/write bits=1; START (SAProvisioningCertificateChain)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (CertificateChainType); next=326
  
  
                  encode_ISO2CertificateChainType(stream, &CertificateUpdateResType.SAProvisioningCertificateChain)?;
                  grammar_id = 326?;
  }
           326=>{
               // Grammar: ID=326; read/write bits=1; START (ContractSignatureCertChain)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (CertificateChainType); next=327
  
  
                  encode_ISO2CertificateChainType(stream, &CertificateUpdateResType.ContractSignatureCertChain)?;
                  grammar_id = 327?;
  }
           327=>{
               // Grammar: ID=327; read/write bits=1; START (ContractSignatureEncryptedPrivateKey)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (privateKeyType); next=328
  
  
                  encode_ISO2ContractSignatureEncryptedPrivateKeyType(stream, &CertificateUpdateResType.ContractSignatureEncryptedPrivateKey)?;
                  grammar_id = 328?;
  }
           328=>{
               // Grammar: ID=328; read/write bits=1; START (DHpublickey)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (dHpublickeyType); next=329
  
  
                  encode_ISO2DiffieHellmanPublickeyType(stream, &CertificateUpdateResType.DHpublickey)?;
                  grammar_id = 329?;
  }
           329=>{
               // Grammar: ID=329; read/write bits=1; START (eMAID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (eMAIDType); next=330
  
  
                  encode_ISO2EMAIDType(stream, &CertificateUpdateResType.eMAID)?;
                  grammar_id = 330?;
  }
           330=>{
               // Grammar: ID=330; read/write bits=2; START (RetryCounter), END Element
              if CertificateUpdateResType.RetryCounter.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (RetryCounter, int); next=3
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_i16( CertificateUpdateResType.RetryCounter as i16)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
  
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}AuthorizationRes; type={urn:iso:15118:2:2013:MsgBody}AuthorizationResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1);
  pub fn encode_ISO2AuthorizationResType(stream: &mut ExiBitstream, AuthorizationResType: &ISO2AuthorizationResType )->Result<(), BitstreamError> {
      let mut grammar_id = 331;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           331=>{
               // Grammar: ID=331; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=332
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, AuthorizationResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 332;
  }
           332=>{
               // Grammar: ID=332; read/write bits=1; START (EVSEProcessing)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, AuthorizationResType.EVSEProcessing)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeteringReceiptReq; type={urn:iso:15118:2:2013:MsgBody}MeteringReceiptReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (0, 1); SessionID, sessionIDType (1, 1); SAScheduleTupleID, SAIDType (0, 1); MeterInfo, MeterInfoType (1, 1);
  pub fn encode_ISO2MeteringReceiptReqType(stream: &mut ExiBitstream, MeteringReceiptReqType: &ISO2MeteringReceiptReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 333;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           333=>{
               // Grammar: ID=333; read/write bits=2; START (Id), START (SessionID)
              if MeteringReceiptReqType.Id.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (Id, NCName); next=334
  
                  // string should not be found in table, so add 2
                   stream.write_u16((MeteringReceiptReqType.Id.len() as u16))?;
                  stream.write_characters(&MeteringReceiptReqType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
                  grammar_id = 334;
                  }
                  }
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (SessionID, hexBinary); next=335
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_u16( MeteringReceiptReqType.SessionID.len() as u16)?;
                      stream.write_bytes( &MeteringReceiptReqType.SessionID)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 335;
  
  
  
  
              }
  }
           334=>{
               // Grammar: ID=334; read/write bits=1; START (SessionID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (hexBinary); next=335
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16( MeteringReceiptReqType.SessionID.len() as u16)?;
                  stream.write_bytes( &MeteringReceiptReqType.SessionID)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 335;
  
  
  
  }
           335=>{
               // Grammar: ID=335; read/write bits=2; START (SAScheduleTupleID), START (MeterInfo)
              if MeteringReceiptReqType.SAScheduleTupleID.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (SAScheduleTupleID, unsignedByte); next=336
  
  
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(8, (MeteringReceiptReqType.SAScheduleTupleID as u32 - 1))?;
                      // encode END Element
                      stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 336;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (MeterInfo, MeterInfoType); next=3
  
  
                      encode_ISO2MeterInfoType(stream, &MeteringReceiptReqType.MeterInfo)?;
                      grammar_id = 3?;
              }
  }
           336=>{
               // Grammar: ID=336; read/write bits=1; START (MeterInfo)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (MeterInfoType); next=3
  
  
                  encode_ISO2MeterInfoType(stream, &MeteringReceiptReqType.MeterInfo)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionRes; type={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1);
  pub fn encode_ISO2PaymentServiceSelectionResType(stream: &mut ExiBitstream, PaymentServiceSelectionResType: &ISO2PaymentServiceSelectionResType )->Result<(), BitstreamError> {
      let mut grammar_id = 337;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           337=>{
               // Grammar: ID=337; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, PaymentServiceSelectionResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeteringReceiptRes; type={urn:iso:15118:2:2013:MsgBody}MeteringReceiptResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (0, 1); DC_EVSEStatus, DC_EVSEStatusType (0, 1); EVSEStatus, EVSEStatusType (0, 1);
  pub fn encode_ISO2MeteringReceiptResType(stream: &mut ExiBitstream, MeteringReceiptResType: &ISO2MeteringReceiptResType )->Result<(), BitstreamError> {
      let mut grammar_id = 338;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           338=>{
               // Grammar: ID=338; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=339
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, MeteringReceiptResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 339;
  }
           339=>{
               // Grammar: ID=339; read/write bits=2; START (AC_EVSEStatus), START (DC_EVSEStatus), START (EVSEStatus)
              if MeteringReceiptResType.AC_EVSEStatus.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (AC_EVSEStatus, EVSEStatusType); next=3
  
  
                      encode_ISO2AC_EVSEStatusType(stream, &MeteringReceiptResType.AC_EVSEStatus)?;
                      grammar_id = 3?;
              }
              else if MeteringReceiptResType.DC_EVSEStatus.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (DC_EVSEStatus, EVSEStatusType); next=3
  
  
                      encode_ISO2DC_EVSEStatusType(stream, &MeteringReceiptResType.DC_EVSEStatus)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (EVSEStatus, EVSEStatusType); next=3
  
  
                      encode_ISO2EVSEStatusType(stream, &MeteringReceiptResType.EVSEStatus)?;
                      grammar_id = 3?;
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateInstallationReq; type={urn:iso:15118:2:2013:MsgBody}CertificateInstallationReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); OEMProvisioningCert, certificateType (1, 1); ListOfRootCertificateIDs, ListOfRootCertificateIDsType (1, 1);
  pub fn encode_ISO2CertificateInstallationReqType(stream: &mut ExiBitstream, CertificateInstallationReqType: &ISO2CertificateInstallationReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 340;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           340=>{
               // Grammar: ID=340; read/write bits=1; START (Id)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (NCName); next=341
  
              // string should not be found in table, so add 2
               stream.write_u16((CertificateInstallationReqType.Id.len() as u16))?;
              stream.write_characters(&CertificateInstallationReqType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
              grammar_id = 341;
              }
              }
  }
           341=>{
               // Grammar: ID=341; read/write bits=1; START (OEMProvisioningCert)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (base64Binary); next=342
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(CertificateInstallationReqType.OEMProvisioningCert.len() as u16)?;
                          stream.write_bytes( &CertificateInstallationReqType.OEMProvisioningCert)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 342;
  
  
  }
           342=>{
               // Grammar: ID=342; read/write bits=1; START (ListOfRootCertificateIDs)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (ListOfRootCertificateIDsType); next=3
  
  
                  encode_ISO2ListOfRootCertificateIDsType(stream, &CertificateInstallationReqType.ListOfRootCertificateIDs)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryRes; type={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1); SAScheduleList, SAScheduleListType (0, 1); SASchedules, SASchedulesType (0, 1); AC_EVSEChargeParameter, AC_EVSEChargeParameterType (0, 1); DC_EVSEChargeParameter, DC_EVSEChargeParameterType (0, 1); EVSEChargeParameter, EVSEChargeParameterType (0, 1);
  pub fn encode_ISO2ChargeParameterDiscoveryResType(stream: &mut ExiBitstream, ChargeParameterDiscoveryResType: &ISO2ChargeParameterDiscoveryResType )->Result<(), BitstreamError> {
      let mut grammar_id = 343;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           343=>{
               // Grammar: ID=343; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=344
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, ChargeParameterDiscoveryResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 344;
  }
           344=>{
               // Grammar: ID=344; read/write bits=1; START (EVSEProcessing)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=345
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, ChargeParameterDiscoveryResType.EVSEProcessing)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 345;
  }
           345=>{
               // Grammar: ID=345; read/write bits=3; START (SAScheduleList), START (SASchedules), START (AC_EVSEChargeParameter), START (DC_EVSEChargeParameter), START (EVSEChargeParameter)
              if ChargeParameterDiscoveryResType.SAScheduleList.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (SAScheduleList, SASchedulesType); next=346
  
  
                      encode_ISO2SAScheduleListType(stream, &ChargeParameterDiscoveryResType.SAScheduleList)?;
                      grammar_id = 346?;
              }
              else if ChargeParameterDiscoveryResType.SASchedules.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Abstract element or type: START (SASchedulesType); next=346
  
  
                      encode_ISO2SASchedulesType(stream, &ChargeParameterDiscoveryResType.SASchedules)?;
                      grammar_id = 346?;
              }
              else if ChargeParameterDiscoveryResType.AC_EVSEChargeParameter.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (AC_EVSEChargeParameter, EVSEChargeParameterType); next=3
  
  
                      encode_ISO2AC_EVSEChargeParameterType(stream, &ChargeParameterDiscoveryResType.AC_EVSEChargeParameter)?;
                      grammar_id = 3?;
              }
              else if ChargeParameterDiscoveryResType.DC_EVSEChargeParameter.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (DC_EVSEChargeParameter, EVSEChargeParameterType); next=3
  
  
                      encode_ISO2DC_EVSEChargeParameterType(stream, &ChargeParameterDiscoveryResType.DC_EVSEChargeParameter)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(3, 4)?;
                  // Abstract element or type: START (EVSEChargeParameterType); next=3
  
  
                      encode_ISO2EVSEChargeParameterType(stream, &ChargeParameterDiscoveryResType.EVSEChargeParameter)?;
                      grammar_id = 3?;
              }
  }
           346=>{
               // Grammar: ID=346; read/write bits=2; START (AC_EVSEChargeParameter), START (DC_EVSEChargeParameter), START (EVSEChargeParameter)
              if ChargeParameterDiscoveryResType.AC_EVSEChargeParameter.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (AC_EVSEChargeParameter, EVSEChargeParameterType); next=3
  
  
                      encode_ISO2AC_EVSEChargeParameterType(stream, &ChargeParameterDiscoveryResType.AC_EVSEChargeParameter)?;
                      grammar_id = 3?;
              }
              else if ChargeParameterDiscoveryResType.DC_EVSEChargeParameter.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (DC_EVSEChargeParameter, EVSEChargeParameterType); next=3
  
  
                      encode_ISO2DC_EVSEChargeParameterType(stream, &ChargeParameterDiscoveryResType.DC_EVSEChargeParameter)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Abstract element or type: START (EVSEChargeParameterType); next=3
  
  
                      encode_ISO2EVSEChargeParameterType(stream, &ChargeParameterDiscoveryResType.EVSEChargeParameter)?;
                      grammar_id = 3?;
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}WeldingDetectionRes; type={urn:iso:15118:2:2013:MsgBody}WeldingDetectionResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1);
  pub fn encode_ISO2WeldingDetectionResType(stream: &mut ExiBitstream, WeldingDetectionResType: &ISO2WeldingDetectionResType )->Result<(), BitstreamError> {
      let mut grammar_id = 347;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           347=>{
               // Grammar: ID=347; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=348
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, WeldingDetectionResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 348;
  }
           348=>{
               // Grammar: ID=348; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVSEStatusType); next=349
  
  
                  encode_ISO2DC_EVSEStatusType(stream, &WeldingDetectionResType.DC_EVSEStatus)?;
                  grammar_id = 349?;
  }
           349=>{
               // Grammar: ID=349; read/write bits=1; START (EVSEPresentVoltage)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=3
  
  
                  encode_ISO2PhysicalValueType(stream, &WeldingDetectionResType.EVSEPresentVoltage)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}WeldingDetectionReq; type={urn:iso:15118:2:2013:MsgBody}WeldingDetectionReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1);
  pub fn encode_ISO2WeldingDetectionReqType(stream: &mut ExiBitstream, WeldingDetectionReqType: &ISO2WeldingDetectionReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 350;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           350=>{
               // Grammar: ID=350; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVStatusType); next=3
  
  
                  encode_ISO2DC_EVStatusType(stream, &WeldingDetectionReqType.DC_EVStatus)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentDetailsReq; type={urn:iso:15118:2:2013:MsgBody}PaymentDetailsReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: eMAID, eMAIDType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1);
  pub fn encode_ISO2PaymentDetailsReqType(stream: &mut ExiBitstream, PaymentDetailsReqType: &ISO2PaymentDetailsReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 351;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           351=>{
               // Grammar: ID=351; read/write bits=1; START (eMAID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=352
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
                   stream.write_u16((PaymentDetailsReqType.eMAID.len() as u16))?;
                  stream.write_characters(&PaymentDetailsReqType.eMAID.to_string(), ISO2eMAID_CHARACTER_SIZE)?;
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 352;
  }
           352=>{
               // Grammar: ID=352; read/write bits=1; START (ContractSignatureCertChain)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (CertificateChainType); next=3
  
  
                  encode_ISO2CertificateChainType(stream, &PaymentDetailsReqType.ContractSignatureCertChain)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PowerDeliveryReq; type={urn:iso:15118:2:2013:MsgBody}PowerDeliveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ChargeProgress, chargeProgressType (1, 1); SAScheduleTupleID, SAIDType (1, 1); ChargingProfile, ChargingProfileType (0, 1); DC_EVPowerDeliveryParameter, DC_EVPowerDeliveryParameterType (0, 1); EVPowerDeliveryParameter, EVPowerDeliveryParameterType (0, 1);
  pub fn encode_ISO2PowerDeliveryReqType(stream: &mut ExiBitstream, PowerDeliveryReqType: &ISO2PowerDeliveryReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 353;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           353=>{
               // Grammar: ID=353; read/write bits=1; START (ChargeProgress)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=354
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(2, PowerDeliveryReqType.ChargeProgress)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 354;
  }
           354=>{
               // Grammar: ID=354; read/write bits=1; START (SAScheduleTupleID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedByte); next=355
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(8, (PowerDeliveryReqType.SAScheduleTupleID as u32 - 1))?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 355;
  
  
  }
           355=>{
               // Grammar: ID=355; read/write bits=3; START (ChargingProfile), START (DC_EVPowerDeliveryParameter), START (EVPowerDeliveryParameter), END Element
              if PowerDeliveryReqType.ChargingProfile.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (ChargingProfile, ChargingProfileType); next=356
  
  
                      encode_ISO2ChargingProfileType(stream, &PowerDeliveryReqType.ChargingProfile)?;
                      grammar_id = 356?;
              }
              else if PowerDeliveryReqType.DC_EVPowerDeliveryParameter.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (DC_EVPowerDeliveryParameter, EVPowerDeliveryParameterType); next=3
  
  
                      encode_ISO2DC_EVPowerDeliveryParameterType(stream, &PowerDeliveryReqType.DC_EVPowerDeliveryParameter)?;
                      grammar_id = 3?;
              }
              else if PowerDeliveryReqType.EVPowerDeliveryParameter.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Abstract element or type: START (EVPowerDeliveryParameterType); next=3
  
  
                      encode_ISO2EVPowerDeliveryParameterType(stream, &PowerDeliveryReqType.EVPowerDeliveryParameter)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           356=>{
               // Grammar: ID=356; read/write bits=2; START (DC_EVPowerDeliveryParameter), START (EVPowerDeliveryParameter), END Element
              if PowerDeliveryReqType.DC_EVPowerDeliveryParameter.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (DC_EVPowerDeliveryParameter, EVPowerDeliveryParameterType); next=3
  
  
                      encode_ISO2DC_EVPowerDeliveryParameterType(stream, &PowerDeliveryReqType.DC_EVPowerDeliveryParameter)?;
                      grammar_id = 3?;
              }
              else if PowerDeliveryReqType.EVPowerDeliveryParameter.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Abstract element or type: START (EVPowerDeliveryParameterType); next=3
  
  
                      encode_ISO2EVPowerDeliveryParameterType(stream, &PowerDeliveryReqType.EVPowerDeliveryParameter)?;
                      grammar_id = 3?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CurrentDemandRes; type={urn:iso:15118:2:2013:MsgBody}CurrentDemandResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1); EVSEPresentCurrent, PhysicalValueType (1, 1); EVSECurrentLimitAchieved, boolean (1, 1); EVSEVoltageLimitAchieved, boolean (1, 1); EVSEPowerLimitAchieved, boolean (1, 1); EVSEMaximumVoltageLimit, PhysicalValueType (0, 1); EVSEMaximumCurrentLimit, PhysicalValueType (0, 1); EVSEMaximumPowerLimit, PhysicalValueType (0, 1); EVSEID, evseIDType (1, 1); SAScheduleTupleID, SAIDType (1, 1); MeterInfo, MeterInfoType (0, 1); ReceiptRequired, boolean (0, 1);
  pub fn encode_ISO2CurrentDemandResType(stream: &mut ExiBitstream, CurrentDemandResType: &ISO2CurrentDemandResType )->Result<(), BitstreamError> {
      let mut grammar_id = 357;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           357=>{
               // Grammar: ID=357; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=358
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, CurrentDemandResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 358;
  }
           358=>{
               // Grammar: ID=358; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVSEStatusType); next=359
  
  
                  encode_ISO2DC_EVSEStatusType(stream, &CurrentDemandResType.DC_EVSEStatus)?;
                  grammar_id = 359?;
  }
           359=>{
               // Grammar: ID=359; read/write bits=1; START (EVSEPresentVoltage)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=360
  
  
                  encode_ISO2PhysicalValueType(stream, &CurrentDemandResType.EVSEPresentVoltage)?;
                  grammar_id = 360?;
  }
           360=>{
               // Grammar: ID=360; read/write bits=1; START (EVSEPresentCurrent)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=361
  
  
                  encode_ISO2PhysicalValueType(stream, &CurrentDemandResType.EVSEPresentCurrent)?;
                  grammar_id = 361?;
  }
           361=>{
               // Grammar: ID=361; read/write bits=1; START (EVSECurrentLimitAchieved)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (boolean); next=362
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( CurrentDemandResType.EVSECurrentLimitAchieved)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 362;
  
  
  }
           362=>{
               // Grammar: ID=362; read/write bits=1; START (EVSEVoltageLimitAchieved)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (boolean); next=363
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( CurrentDemandResType.EVSEVoltageLimitAchieved)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 363;
  
  
  }
           363=>{
               // Grammar: ID=363; read/write bits=1; START (EVSEPowerLimitAchieved)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (boolean); next=364
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( CurrentDemandResType.EVSEPowerLimitAchieved)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 364;
  
  
  }
           364=>{
               // Grammar: ID=364; read/write bits=3; START (EVSEMaximumVoltageLimit), START (EVSEMaximumCurrentLimit), START (EVSEMaximumPowerLimit), START (EVSEID)
              if CurrentDemandResType.EVSEMaximumVoltageLimit.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (EVSEMaximumVoltageLimit, PhysicalValueType); next=365
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandResType.EVSEMaximumVoltageLimit)?;
                      grammar_id = 365?;
              }
              else if CurrentDemandResType.EVSEMaximumCurrentLimit.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (EVSEMaximumCurrentLimit, PhysicalValueType); next=366
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandResType.EVSEMaximumCurrentLimit)?;
                      grammar_id = 366?;
              }
              else if CurrentDemandResType.EVSEMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (EVSEMaximumPowerLimit, PhysicalValueType); next=367
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandResType.EVSEMaximumPowerLimit)?;
                      grammar_id = 367?;
              }
              else
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (EVSEID, string); next=368
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((CurrentDemandResType.EVSEID.len() as u16))?;
                      stream.write_characters(&CurrentDemandResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 368;
              }
  }
           365=>{
               // Grammar: ID=365; read/write bits=2; START (EVSEMaximumCurrentLimit), START (EVSEMaximumPowerLimit), START (EVSEID)
              if CurrentDemandResType.EVSEMaximumCurrentLimit.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (EVSEMaximumCurrentLimit, PhysicalValueType); next=366
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandResType.EVSEMaximumCurrentLimit)?;
                      grammar_id = 366?;
              }
              else if CurrentDemandResType.EVSEMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (EVSEMaximumPowerLimit, PhysicalValueType); next=367
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandResType.EVSEMaximumPowerLimit)?;
                      grammar_id = 367?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (EVSEID, string); next=368
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((CurrentDemandResType.EVSEID.len() as u16))?;
                      stream.write_characters(&CurrentDemandResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 368;
              }
  }
           366=>{
               // Grammar: ID=366; read/write bits=2; START (EVSEMaximumPowerLimit), START (EVSEID)
              if CurrentDemandResType.EVSEMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (EVSEMaximumPowerLimit, PhysicalValueType); next=367
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandResType.EVSEMaximumPowerLimit)?;
                      grammar_id = 367?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (EVSEID, string); next=368
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((CurrentDemandResType.EVSEID.len() as u16))?;
                      stream.write_characters(&CurrentDemandResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 368;
              }
  }
           367=>{
               // Grammar: ID=367; read/write bits=1; START (EVSEID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=368
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
                   stream.write_u16((CurrentDemandResType.EVSEID.len() as u16))?;
                  stream.write_characters(&CurrentDemandResType.EVSEID.to_string(), ISO2EVSEID_CHARACTER_SIZE)?;
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 368;
  }
           368=>{
               // Grammar: ID=368; read/write bits=1; START (SAScheduleTupleID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedByte); next=369
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(8, (CurrentDemandResType.SAScheduleTupleID as u32 - 1))?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 369;
  
  
  }
           369=>{
               // Grammar: ID=369; read/write bits=2; START (MeterInfo), START (ReceiptRequired), END Element
              if CurrentDemandResType.MeterInfo.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (MeterInfo, MeterInfoType); next=370
  
  
                      encode_ISO2MeterInfoType(stream, &CurrentDemandResType.MeterInfo)?;
                      grammar_id = 370?;
              }
              else if CurrentDemandResType.ReceiptRequired.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (ReceiptRequired, boolean); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandResType.ReceiptRequired)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           370=>{
               // Grammar: ID=370; read/write bits=2; START (ReceiptRequired), END Element
              if CurrentDemandResType.ReceiptRequired.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ReceiptRequired, boolean); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandResType.ReceiptRequired)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 3;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryReq; type={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ServiceScope, serviceScopeType (0, 1); ServiceCategory, serviceCategoryType (0, 1);
  pub fn encode_ISO2ServiceDiscoveryReqType(stream: &mut ExiBitstream, ServiceDiscoveryReqType: &ISO2ServiceDiscoveryReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 371;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           371=>{
               // Grammar: ID=371; read/write bits=2; START (ServiceScope), START (ServiceCategory), END Element
              if ServiceDiscoveryReqType.ServiceScope.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ServiceScope, string); next=372
  
                      stream.write_nbit_uint(1, 0)?;
                      // string should not be found in table, so add 2
                       stream.write_u16((ServiceDiscoveryReqType.ServiceScope.len() as u16))?;
                      stream.write_characters(&ServiceDiscoveryReqType.ServiceScope.to_string(), ISO2ServiceScope_CHARACTER_SIZE)?;
                      // encode END Element
                       stream.write_nbit_uint(1, 0)?;
                      grammar_id = 372;
              }
              else if ServiceDiscoveryReqType.ServiceCategory.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (ServiceCategory, string); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(2, ServiceDiscoveryReqType.ServiceCategory)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           372=>{
               // Grammar: ID=372; read/write bits=2; START (ServiceCategory), END Element
              if ServiceDiscoveryReqType.ServiceCategory.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (ServiceCategory, string); next=3
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_nbit_uint(2, ServiceDiscoveryReqType.ServiceCategory)?;
                      // encode END Element
                       stream.write_nbit_uint( 1, 0)?;
                      grammar_id = 3;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: END Element; next=4
                  done = True;
                  grammar_id = 4;
                  }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentDetailsRes; type={urn:iso:15118:2:2013:MsgBody}PaymentDetailsResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); GenChallenge, genChallengeType (1, 1); EVSETimeStamp, long (1, 1);
  pub fn encode_ISO2PaymentDetailsResType(stream: &mut ExiBitstream, PaymentDetailsResType: &ISO2PaymentDetailsResType )->Result<(), BitstreamError> {
      let mut grammar_id = 373;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           373=>{
               // Grammar: ID=373; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=374
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, PaymentDetailsResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 374;
  }
           374=>{
               // Grammar: ID=374; read/write bits=1; START (GenChallenge)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (base64Binary); next=375
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(PaymentDetailsResType.GenChallenge.len() as u16)?;
                          stream.write_bytes( &PaymentDetailsResType.GenChallenge)?;
                              // encode END Element
                              stream.write_nbit_uint(1, 0)?;
                              grammar_id = 375;
  
  
  }
           375=>{
               // Grammar: ID=375; read/write bits=1; START (EVSETimeStamp)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (integer); next=3
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_i64(PaymentDetailsResType.EVSETimeStamp)?;
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionStopRes; type={urn:iso:15118:2:2013:MsgBody}SessionStopResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1);
  pub fn encode_ISO2SessionStopResType(stream: &mut ExiBitstream, SessionStopResType: &ISO2SessionStopResType )->Result<(), BitstreamError> {
      let mut grammar_id = 376;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           376=>{
               // Grammar: ID=376; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=3
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, SessionStopResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateUpdateReq; type={urn:iso:15118:2:2013:MsgBody}CertificateUpdateReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); eMAID, eMAIDType (1, 1); ListOfRootCertificateIDs, ListOfRootCertificateIDsType (1, 1);
  pub fn encode_ISO2CertificateUpdateReqType(stream: &mut ExiBitstream, CertificateUpdateReqType: &ISO2CertificateUpdateReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 377;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           377=>{
               // Grammar: ID=377; read/write bits=1; START (Id)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (NCName); next=378
  
              // string should not be found in table, so add 2
               stream.write_u16((CertificateUpdateReqType.Id.len() as u16))?;
              stream.write_characters(&CertificateUpdateReqType.Id.to_string(), ISO2Id_CHARACTER_SIZE)?;
              grammar_id = 378;
              }
              }
  }
           378=>{
               // Grammar: ID=378; read/write bits=1; START (ContractSignatureCertChain)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (CertificateChainType); next=379
  
  
                  encode_ISO2CertificateChainType(stream, &CertificateUpdateReqType.ContractSignatureCertChain)?;
                  grammar_id = 379?;
  }
           379=>{
               // Grammar: ID=379; read/write bits=1; START (eMAID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=380
  
                  stream.write_nbit_uint(1, 0)?;
                  // string should not be found in table, so add 2
                   stream.write_u16((CertificateUpdateReqType.eMAID.len() as u16))?;
                  stream.write_characters(&CertificateUpdateReqType.eMAID.to_string(), ISO2eMAID_CHARACTER_SIZE)?;
                  // encode END Element
                   stream.write_nbit_uint(1, 0)?;
                  grammar_id = 380;
  }
           380=>{
               // Grammar: ID=380; read/write bits=1; START (ListOfRootCertificateIDs)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (ListOfRootCertificateIDsType); next=3
  
  
                  encode_ISO2ListOfRootCertificateIDsType(stream, &CertificateUpdateReqType.ListOfRootCertificateIDs)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDetailReq; type={urn:iso:15118:2:2013:MsgBody}ServiceDetailReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ServiceID, serviceIDType (1, 1);
  pub fn encode_ISO2ServiceDetailReqType(stream: &mut ExiBitstream, ServiceDetailReqType: &ISO2ServiceDetailReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 381;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           381=>{
               // Grammar: ID=381; read/write bits=1; START (ServiceID)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (unsignedShort); next=3
  
  
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_u16(ServiceDetailReqType.ServiceID as u16)?;
                  // encode END Element
                  stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 3;
  
  
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CurrentDemandReq; type={urn:iso:15118:2:2013:MsgBody}CurrentDemandReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1); EVTargetCurrent, PhysicalValueType (1, 1); EVMaximumVoltageLimit, PhysicalValueType (0, 1); EVMaximumCurrentLimit, PhysicalValueType (0, 1); EVMaximumPowerLimit, PhysicalValueType (0, 1); BulkChargingComplete, boolean (0, 1); ChargingComplete, boolean (1, 1); RemainingTimeToFullSoC, PhysicalValueType (0, 1); RemainingTimeToBulkSoC, PhysicalValueType (0, 1); EVTargetVoltage, PhysicalValueType (1, 1);
  pub fn encode_ISO2CurrentDemandReqType(stream: &mut ExiBitstream, CurrentDemandReqType: &ISO2CurrentDemandReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 382;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           382=>{
               // Grammar: ID=382; read/write bits=1; START (DC_EVStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVStatusType); next=383
  
  
                  encode_ISO2DC_EVStatusType(stream, &CurrentDemandReqType.DC_EVStatus)?;
                  grammar_id = 383?;
  }
           383=>{
               // Grammar: ID=383; read/write bits=1; START (EVTargetCurrent)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=384
  
  
                  encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.EVTargetCurrent)?;
                  grammar_id = 384?;
  }
           384=>{
               // Grammar: ID=384; read/write bits=3; START (EVMaximumVoltageLimit), START (EVMaximumCurrentLimit), START (EVMaximumPowerLimit), START (BulkChargingComplete), START (ChargingComplete)
              if CurrentDemandReqType.EVMaximumVoltageLimit.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (EVMaximumVoltageLimit, PhysicalValueType); next=385
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.EVMaximumVoltageLimit)?;
                      grammar_id = 385?;
              }
              else if CurrentDemandReqType.EVMaximumCurrentLimit.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (EVMaximumCurrentLimit, PhysicalValueType); next=386
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.EVMaximumCurrentLimit)?;
                      grammar_id = 386?;
              }
              else if CurrentDemandReqType.EVMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (EVMaximumPowerLimit, PhysicalValueType); next=387
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.EVMaximumPowerLimit)?;
                      grammar_id = 387?;
              }
              else if CurrentDemandReqType.BulkChargingComplete.is_some()
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (BulkChargingComplete, boolean); next=388
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.BulkChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 388;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 4)?;
                  // Event: START (ChargingComplete, boolean); next=389
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.ChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 389;
  
  
  
              }
  }
           385=>{
               // Grammar: ID=385; read/write bits=3; START (EVMaximumCurrentLimit), START (EVMaximumPowerLimit), START (BulkChargingComplete), START (ChargingComplete)
              if CurrentDemandReqType.EVMaximumCurrentLimit.is_some()
              {
                  stream.write_nbit_uint(3, 0)?;
                  // Event: START (EVMaximumCurrentLimit, PhysicalValueType); next=386
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.EVMaximumCurrentLimit)?;
                      grammar_id = 386?;
              }
              else if CurrentDemandReqType.EVMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(3, 1)?;
                  // Event: START (EVMaximumPowerLimit, PhysicalValueType); next=387
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.EVMaximumPowerLimit)?;
                      grammar_id = 387?;
              }
              else if CurrentDemandReqType.BulkChargingComplete.is_some()
              {
                  stream.write_nbit_uint(3, 2)?;
                  // Event: START (BulkChargingComplete, boolean); next=388
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.BulkChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 388;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(3, 3)?;
                  // Event: START (ChargingComplete, boolean); next=389
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.ChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 389;
  
  
  
              }
  }
           386=>{
               // Grammar: ID=386; read/write bits=2; START (EVMaximumPowerLimit), START (BulkChargingComplete), START (ChargingComplete)
              if CurrentDemandReqType.EVMaximumPowerLimit.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (EVMaximumPowerLimit, PhysicalValueType); next=387
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.EVMaximumPowerLimit)?;
                      grammar_id = 387?;
              }
              else if CurrentDemandReqType.BulkChargingComplete.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (BulkChargingComplete, boolean); next=388
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.BulkChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 388;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (ChargingComplete, boolean); next=389
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.ChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 389;
  
  
  
              }
  }
           387=>{
               // Grammar: ID=387; read/write bits=2; START (BulkChargingComplete), START (ChargingComplete)
              if CurrentDemandReqType.BulkChargingComplete.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (BulkChargingComplete, boolean); next=388
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.BulkChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 388;
  
  
  
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (ChargingComplete, boolean); next=389
                      stream.write_nbit_uint(1, 0)?;
                      stream.write_bool( CurrentDemandReqType.ChargingComplete)?;
                          // encode END Element
                          stream.write_nbit_uint(1, 0)?;
                              grammar_id = 389;
  
  
  
              }
  }
           388=>{
               // Grammar: ID=388; read/write bits=1; START (ChargingComplete)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (boolean); next=389
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_bool( CurrentDemandReqType.ChargingComplete)?;
                      // encode END Element
                      stream.write_nbit_uint(1, 0)?;
                          grammar_id = 389;
  
  
  }
           389=>{
               // Grammar: ID=389; read/write bits=2; START (RemainingTimeToFullSoC), START (RemainingTimeToBulkSoC), START (EVTargetVoltage)
              if CurrentDemandReqType.RemainingTimeToFullSoC.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (RemainingTimeToFullSoC, PhysicalValueType); next=390
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.RemainingTimeToFullSoC)?;
                      grammar_id = 390?;
              }
              else if CurrentDemandReqType.RemainingTimeToBulkSoC.is_some()
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (RemainingTimeToBulkSoC, PhysicalValueType); next=391
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.RemainingTimeToBulkSoC)?;
                      grammar_id = 391?;
              }
              else
              {
                  stream.write_nbit_uint(2, 2)?;
                  // Event: START (EVTargetVoltage, PhysicalValueType); next=3
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.EVTargetVoltage)?;
                      grammar_id = 3?;
              }
  }
           390=>{
               // Grammar: ID=390; read/write bits=2; START (RemainingTimeToBulkSoC), START (EVTargetVoltage)
              if CurrentDemandReqType.RemainingTimeToBulkSoC.is_some()
              {
                  stream.write_nbit_uint(2, 0)?;
                  // Event: START (RemainingTimeToBulkSoC, PhysicalValueType); next=391
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.RemainingTimeToBulkSoC)?;
                      grammar_id = 391?;
              }
              else
              {
                  stream.write_nbit_uint(2, 1)?;
                  // Event: START (EVTargetVoltage, PhysicalValueType); next=3
  
  
                      encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.EVTargetVoltage)?;
                      grammar_id = 3?;
              }
  }
           391=>{
               // Grammar: ID=391; read/write bits=1; START (EVTargetVoltage)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=3
  
  
                  encode_ISO2PhysicalValueType(stream, &CurrentDemandReqType.EVTargetVoltage)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PreChargeRes; type={urn:iso:15118:2:2013:MsgBody}PreChargeResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1);
  pub fn encode_ISO2PreChargeResType(stream: &mut ExiBitstream, PreChargeResType: &ISO2PreChargeResType )->Result<(), BitstreamError> {
      let mut grammar_id = 392;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           392=>{
               // Grammar: ID=392; read/write bits=1; START (ResponseCode)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (string); next=393
                  stream.write_nbit_uint(1, 0)?;
                  stream.write_nbit_uint(5, PreChargeResType.ResponseCode)?;
                  // encode END Element
                   stream.write_nbit_uint( 1, 0)?;
                  grammar_id = 393;
  }
           393=>{
               // Grammar: ID=393; read/write bits=1; START (DC_EVSEStatus)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (EVSEStatusType); next=394
  
  
                  encode_ISO2DC_EVSEStatusType(stream, &PreChargeResType.DC_EVSEStatus)?;
                  grammar_id = 394?;
  }
           394=>{
               // Grammar: ID=394; read/write bits=1; START (EVSEPresentVoltage)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (PhysicalValueType); next=3
  
  
                  encode_ISO2PhysicalValueType(stream, &PreChargeResType.EVSEPresentVoltage)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionSetupReq; type={urn:iso:15118:2:2013:MsgBody}SessionSetupReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: EVCCID, evccIDType (1, 1);
  pub fn encode_ISO2SessionSetupReqType(stream: &mut ExiBitstream, SessionSetupReqType: &ISO2SessionSetupReqType )->Result<(), BitstreamError> {
      let mut grammar_id = 395;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           395=>{
               // Grammar: ID=395; read/write bits=1; START (EVCCID)
              stream.write_nbit_uint( 1, 0)?;
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
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}Body; type={urn:iso:15118:2:2013:MsgBody}BodyType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: AuthorizationReq, AuthorizationReqType (0, 1); AuthorizationRes, AuthorizationResType (0, 1); BodyElement, BodyBaseType (0, 1); CableCheckReq, CableCheckReqType (0, 1); CableCheckRes, CableCheckResType (0, 1); CertificateInstallationReq, CertificateInstallationReqType (0, 1); CertificateInstallationRes, CertificateInstallationResType (0, 1); CertificateUpdateReq, CertificateUpdateReqType (0, 1); CertificateUpdateRes, CertificateUpdateResType (0, 1); ChargeParameterDiscoveryReq, ChargeParameterDiscoveryReqType (0, 1); ChargeParameterDiscoveryRes, ChargeParameterDiscoveryResType (0, 1); ChargingStatusReq, ChargingStatusReqType (0, 1); ChargingStatusRes, ChargingStatusResType (0, 1); CurrentDemandReq, CurrentDemandReqType (0, 1); CurrentDemandRes, CurrentDemandResType (0, 1); MeteringReceiptReq, MeteringReceiptReqType (0, 1); MeteringReceiptRes, MeteringReceiptResType (0, 1); PaymentDetailsReq, PaymentDetailsReqType (0, 1); PaymentDetailsRes, PaymentDetailsResType (0, 1); PaymentServiceSelectionReq, PaymentServiceSelectionReqType (0, 1); PaymentServiceSelectionRes, PaymentServiceSelectionResType (0, 1); PowerDeliveryReq, PowerDeliveryReqType (0, 1); PowerDeliveryRes, PowerDeliveryResType (0, 1); PreChargeReq, PreChargeReqType (0, 1); PreChargeRes, PreChargeResType (0, 1); ServiceDetailReq, ServiceDetailReqType (0, 1); ServiceDetailRes, ServiceDetailResType (0, 1); ServiceDiscoveryReq, ServiceDiscoveryReqType (0, 1); ServiceDiscoveryRes, ServiceDiscoveryResType (0, 1); SessionSetupReq, SessionSetupReqType (0, 1); SessionSetupRes, SessionSetupResType (0, 1); SessionStopReq, SessionStopReqType (0, 1); SessionStopRes, SessionStopResType (0, 1); WeldingDetectionReq, WeldingDetectionReqType (0, 1); WeldingDetectionRes, WeldingDetectionResType (0, 1);
  pub fn encode_ISO2BodyType(stream: &mut ExiBitstream, BodyType: &ISO2BodyType )->Result<(), BitstreamError> {
      let mut grammar_id = 396;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           396=>{
               // Grammar: ID=396; read/write bits=6; START (AuthorizationReq)
              if BodyType.AuthorizationReq.is_some()
              {
                  stream.write_nbit_uint(6, 0)?;
                  // Event: AuthorizationReq
  
  
                      encode_ISO2AuthorizationReqType(stream, &BodyType.AuthorizationReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.AuthorizationRes.is_some()
              {
                  stream.write_nbit_uint(6, 1)?;
                  // Event: AuthorizationRes
  
  
                      encode_ISO2AuthorizationResType(stream, &BodyType.AuthorizationRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.BodyElement.is_some()
              {
                  stream.write_nbit_uint(6, 2)?;
                  // Event: BodyElement
  
  
                      encode_ISO2BodyBaseType(stream, &BodyType.BodyElement)?;
                      grammar_id = 3?;
              }
              else if BodyType.CableCheckReq.is_some()
              {
                  stream.write_nbit_uint(6, 3)?;
                  // Event: CableCheckReq
  
  
                      encode_ISO2CableCheckReqType(stream, &BodyType.CableCheckReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.CableCheckRes.is_some()
              {
                  stream.write_nbit_uint(6, 4)?;
                  // Event: CableCheckRes
  
  
                      encode_ISO2CableCheckResType(stream, &BodyType.CableCheckRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.CertificateInstallationReq.is_some()
              {
                  stream.write_nbit_uint(6, 5)?;
                  // Event: CertificateInstallationReq
  
  
                      encode_ISO2CertificateInstallationReqType(stream, &BodyType.CertificateInstallationReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.CertificateInstallationRes.is_some()
              {
                  stream.write_nbit_uint(6, 6)?;
                  // Event: CertificateInstallationRes
  
  
                      encode_ISO2CertificateInstallationResType(stream, &BodyType.CertificateInstallationRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.CertificateUpdateReq.is_some()
              {
                  stream.write_nbit_uint(6, 7)?;
                  // Event: CertificateUpdateReq
  
  
                      encode_ISO2CertificateUpdateReqType(stream, &BodyType.CertificateUpdateReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.CertificateUpdateRes.is_some()
              {
                  stream.write_nbit_uint(6, 8)?;
                  // Event: CertificateUpdateRes
  
  
                      encode_ISO2CertificateUpdateResType(stream, &BodyType.CertificateUpdateRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.ChargeParameterDiscoveryReq.is_some()
              {
                  stream.write_nbit_uint(6, 9)?;
                  // Event: ChargeParameterDiscoveryReq
  
  
                      encode_ISO2ChargeParameterDiscoveryReqType(stream, &BodyType.ChargeParameterDiscoveryReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.ChargeParameterDiscoveryRes.is_some()
              {
                  stream.write_nbit_uint(6, 10)?;
                  // Event: ChargeParameterDiscoveryRes
  
  
                      encode_ISO2ChargeParameterDiscoveryResType(stream, &BodyType.ChargeParameterDiscoveryRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.ChargingStatusReq.is_some()
              {
                  stream.write_nbit_uint(6, 11)?;
                  // Event: ChargingStatusReq
  
  
                      encode_ISO2ChargingStatusReqType(stream, &BodyType.ChargingStatusReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.ChargingStatusRes.is_some()
              {
                  stream.write_nbit_uint(6, 12)?;
                  // Event: ChargingStatusRes
  
  
                      encode_ISO2ChargingStatusResType(stream, &BodyType.ChargingStatusRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.CurrentDemandReq.is_some()
              {
                  stream.write_nbit_uint(6, 13)?;
                  // Event: CurrentDemandReq
  
  
                      encode_ISO2CurrentDemandReqType(stream, &BodyType.CurrentDemandReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.CurrentDemandRes.is_some()
              {
                  stream.write_nbit_uint(6, 14)?;
                  // Event: CurrentDemandRes
  
  
                      encode_ISO2CurrentDemandResType(stream, &BodyType.CurrentDemandRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.MeteringReceiptReq.is_some()
              {
                  stream.write_nbit_uint(6, 15)?;
                  // Event: MeteringReceiptReq
  
  
                      encode_ISO2MeteringReceiptReqType(stream, &BodyType.MeteringReceiptReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.MeteringReceiptRes.is_some()
              {
                  stream.write_nbit_uint(6, 16)?;
                  // Event: MeteringReceiptRes
  
  
                      encode_ISO2MeteringReceiptResType(stream, &BodyType.MeteringReceiptRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.PaymentDetailsReq.is_some()
              {
                  stream.write_nbit_uint(6, 17)?;
                  // Event: PaymentDetailsReq
  
  
                      encode_ISO2PaymentDetailsReqType(stream, &BodyType.PaymentDetailsReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.PaymentDetailsRes.is_some()
              {
                  stream.write_nbit_uint(6, 18)?;
                  // Event: PaymentDetailsRes
  
  
                      encode_ISO2PaymentDetailsResType(stream, &BodyType.PaymentDetailsRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.PaymentServiceSelectionReq.is_some()
              {
                  stream.write_nbit_uint(6, 19)?;
                  // Event: PaymentServiceSelectionReq
  
  
                      encode_ISO2PaymentServiceSelectionReqType(stream, &BodyType.PaymentServiceSelectionReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.PaymentServiceSelectionRes.is_some()
              {
                  stream.write_nbit_uint(6, 20)?;
                  // Event: PaymentServiceSelectionRes
  
  
                      encode_ISO2PaymentServiceSelectionResType(stream, &BodyType.PaymentServiceSelectionRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.PowerDeliveryReq.is_some()
              {
                  stream.write_nbit_uint(6, 21)?;
                  // Event: PowerDeliveryReq
  
  
                      encode_ISO2PowerDeliveryReqType(stream, &BodyType.PowerDeliveryReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.PowerDeliveryRes.is_some()
              {
                  stream.write_nbit_uint(6, 22)?;
                  // Event: PowerDeliveryRes
  
  
                      encode_ISO2PowerDeliveryResType(stream, &BodyType.PowerDeliveryRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.PreChargeReq.is_some()
              {
                  stream.write_nbit_uint(6, 23)?;
                  // Event: PreChargeReq
  
  
                      encode_ISO2PreChargeReqType(stream, &BodyType.PreChargeReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.PreChargeRes.is_some()
              {
                  stream.write_nbit_uint(6, 24)?;
                  // Event: PreChargeRes
  
  
                      encode_ISO2PreChargeResType(stream, &BodyType.PreChargeRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.ServiceDetailReq.is_some()
              {
                  stream.write_nbit_uint(6, 25)?;
                  // Event: ServiceDetailReq
  
  
                      encode_ISO2ServiceDetailReqType(stream, &BodyType.ServiceDetailReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.ServiceDetailRes.is_some()
              {
                  stream.write_nbit_uint(6, 26)?;
                  // Event: ServiceDetailRes
  
  
                      encode_ISO2ServiceDetailResType(stream, &BodyType.ServiceDetailRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.ServiceDiscoveryReq.is_some()
              {
                  stream.write_nbit_uint(6, 27)?;
                  // Event: ServiceDiscoveryReq
  
  
                      encode_ISO2ServiceDiscoveryReqType(stream, &BodyType.ServiceDiscoveryReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.ServiceDiscoveryRes.is_some()
              {
                  stream.write_nbit_uint(6, 28)?;
                  // Event: ServiceDiscoveryRes
  
  
                      encode_ISO2ServiceDiscoveryResType(stream, &BodyType.ServiceDiscoveryRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.SessionSetupReq.is_some()
              {
                  stream.write_nbit_uint(6, 29)?;
                  // Event: SessionSetupReq
  
  
                      encode_ISO2SessionSetupReqType(stream, &BodyType.SessionSetupReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.SessionSetupRes.is_some()
              {
                  stream.write_nbit_uint(6, 30)?;
                  // Event: SessionSetupRes
  
  
                      encode_ISO2SessionSetupResType(stream, &BodyType.SessionSetupRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.SessionStopReq.is_some()
              {
                  stream.write_nbit_uint(6, 31)?;
                  // Event: SessionStopReq
  
  
                      encode_ISO2SessionStopReqType(stream, &BodyType.SessionStopReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.SessionStopRes.is_some()
              {
                  stream.write_nbit_uint(6, 32)?;
                  // Event: SessionStopRes
  
  
                      encode_ISO2SessionStopResType(stream, &BodyType.SessionStopRes)?;
                      grammar_id = 3?;
              }
              else if BodyType.WeldingDetectionReq.is_some()
              {
                  stream.write_nbit_uint(6, 33)?;
                  // Event: WeldingDetectionReq
  
  
                      encode_ISO2WeldingDetectionReqType(stream, &BodyType.WeldingDetectionReq)?;
                      grammar_id = 3?;
              }
              else if BodyType.WeldingDetectionRes.is_some()
              {
                  stream.write_nbit_uint(6, 34)?;
                  // Event: WeldingDetectionRes
  
  
                      encode_ISO2WeldingDetectionResType(stream, &BodyType.WeldingDetectionRes)?;
                      grammar_id = 3?;
              }
              else
              {
                  return Err(BitstreamError::UNKNOWNEVENTFORENCODING);
              }
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}V2G_Message; type=AnonymousType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Header, MessageHeaderType (1, 1); Body, BodyType (1, 1);
  pub fn encode_ISO2V2G_Message(stream: &mut ExiBitstream, V2G_Message: &ISO2V2G_Message )->Result<(), BitstreamError> {
      let mut grammar_id = 397;
      let mut done = 0;
  
      while(!done)
      {
          match(grammar_id){
      
           397=>{
               // Grammar: ID=397; read/write bits=1; START (Header)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (MessageHeaderType); next=398
  
  
                  encode_ISO2MessageHeaderType(stream, &V2G_Message.Header)?;
                  grammar_id = 398?;
  }
           398=>{
               // Grammar: ID=398; read/write bits=1; START (Body)
              stream.write_nbit_uint( 1, 0)?;
              // Event: START (BodyType); next=3
  
  
                  encode_ISO2BodyType(stream, &V2G_Message.Body)?;
                  grammar_id = 3?;
  }
           3=>{
               // Grammar: ID=3; read/write bits=1; END Element
              stream.write_nbit_uint(1, 0)?;
              // Event: END Element; next=4
              done = True;
              grammar_id = 4;
              }
  
          _=>{
              return Err(BitstreamError::UnknownGrammarId);
              }
          }
      }
      ok(())
      }
  
  
  
  
  // main function for encoding
  fn encode_ISO2exiDocument(stream: &mut ExiBitstream, exiDoc: &ISO2exiDocument)->Result<(),BitstreamError>{
      stream.write_header()?;
          else
          {
              return Err(BitstreamError::UNKNOWNEVENTFORENCODING);
          }
  }
  
  
  