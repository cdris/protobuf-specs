// This file is generated by rust-protobuf 2.28.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `sigstore_trustroot.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct TransparencyLogInstance {
    // message fields
    pub base_url: ::std::string::String,
    pub hash_algorithm: super::sigstore_common::HashAlgorithm,
    pub public_key: ::protobuf::SingularPtrField<super::sigstore_common::PublicKey>,
    pub log_id: ::protobuf::SingularPtrField<super::sigstore_common::LogId>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TransparencyLogInstance {
    fn default() -> &'a TransparencyLogInstance {
        <TransparencyLogInstance as ::protobuf::Message>::default_instance()
    }
}

impl TransparencyLogInstance {
    pub fn new() -> TransparencyLogInstance {
        ::std::default::Default::default()
    }

    // string base_url = 1;


    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
    pub fn clear_base_url(&mut self) {
        self.base_url.clear();
    }

    // Param is passed by value, moved
    pub fn set_base_url(&mut self, v: ::std::string::String) {
        self.base_url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base_url(&mut self) -> &mut ::std::string::String {
        &mut self.base_url
    }

    // Take field
    pub fn take_base_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.base_url, ::std::string::String::new())
    }

    // .dev.sigstore.common.v1.HashAlgorithm hash_algorithm = 2;


    pub fn get_hash_algorithm(&self) -> super::sigstore_common::HashAlgorithm {
        self.hash_algorithm
    }
    pub fn clear_hash_algorithm(&mut self) {
        self.hash_algorithm = super::sigstore_common::HashAlgorithm::HASH_ALGORITHM_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_hash_algorithm(&mut self, v: super::sigstore_common::HashAlgorithm) {
        self.hash_algorithm = v;
    }

    // .dev.sigstore.common.v1.PublicKey public_key = 3;


    pub fn get_public_key(&self) -> &super::sigstore_common::PublicKey {
        self.public_key.as_ref().unwrap_or_else(|| <super::sigstore_common::PublicKey as ::protobuf::Message>::default_instance())
    }
    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: super::sigstore_common::PublicKey) {
        self.public_key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut super::sigstore_common::PublicKey {
        if self.public_key.is_none() {
            self.public_key.set_default();
        }
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> super::sigstore_common::PublicKey {
        self.public_key.take().unwrap_or_else(|| super::sigstore_common::PublicKey::new())
    }

    // .dev.sigstore.common.v1.LogId log_id = 4;


    pub fn get_log_id(&self) -> &super::sigstore_common::LogId {
        self.log_id.as_ref().unwrap_or_else(|| <super::sigstore_common::LogId as ::protobuf::Message>::default_instance())
    }
    pub fn clear_log_id(&mut self) {
        self.log_id.clear();
    }

    pub fn has_log_id(&self) -> bool {
        self.log_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_log_id(&mut self, v: super::sigstore_common::LogId) {
        self.log_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log_id(&mut self) -> &mut super::sigstore_common::LogId {
        if self.log_id.is_none() {
            self.log_id.set_default();
        }
        self.log_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_log_id(&mut self) -> super::sigstore_common::LogId {
        self.log_id.take().unwrap_or_else(|| super::sigstore_common::LogId::new())
    }
}

impl ::protobuf::Message for TransparencyLogInstance {
    fn is_initialized(&self) -> bool {
        for v in &self.public_key {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.log_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.base_url)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.hash_algorithm, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.public_key)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.log_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.base_url.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.base_url);
        }
        if self.hash_algorithm != super::sigstore_common::HashAlgorithm::HASH_ALGORITHM_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(2, self.hash_algorithm);
        }
        if let Some(ref v) = self.public_key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.log_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.base_url.is_empty() {
            os.write_string(1, &self.base_url)?;
        }
        if self.hash_algorithm != super::sigstore_common::HashAlgorithm::HASH_ALGORITHM_UNSPECIFIED {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.hash_algorithm))?;
        }
        if let Some(ref v) = self.public_key.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.log_id.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TransparencyLogInstance {
        TransparencyLogInstance::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "base_url",
                |m: &TransparencyLogInstance| { &m.base_url },
                |m: &mut TransparencyLogInstance| { &mut m.base_url },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::sigstore_common::HashAlgorithm>>(
                "hash_algorithm",
                |m: &TransparencyLogInstance| { &m.hash_algorithm },
                |m: &mut TransparencyLogInstance| { &mut m.hash_algorithm },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::sigstore_common::PublicKey>>(
                "public_key",
                |m: &TransparencyLogInstance| { &m.public_key },
                |m: &mut TransparencyLogInstance| { &mut m.public_key },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::sigstore_common::LogId>>(
                "log_id",
                |m: &TransparencyLogInstance| { &m.log_id },
                |m: &mut TransparencyLogInstance| { &mut m.log_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TransparencyLogInstance>(
                "TransparencyLogInstance",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TransparencyLogInstance {
        static instance: ::protobuf::rt::LazyV2<TransparencyLogInstance> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TransparencyLogInstance::new)
    }
}

impl ::protobuf::Clear for TransparencyLogInstance {
    fn clear(&mut self) {
        self.base_url.clear();
        self.hash_algorithm = super::sigstore_common::HashAlgorithm::HASH_ALGORITHM_UNSPECIFIED;
        self.public_key.clear();
        self.log_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransparencyLogInstance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransparencyLogInstance {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CertificateAuthority {
    // message fields
    pub subject: ::protobuf::SingularPtrField<super::sigstore_common::DistinguishedName>,
    pub uri: ::std::string::String,
    pub cert_chain: ::protobuf::SingularPtrField<super::sigstore_common::X509CertificateChain>,
    pub valid_for: ::protobuf::SingularPtrField<super::sigstore_common::TimeRange>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CertificateAuthority {
    fn default() -> &'a CertificateAuthority {
        <CertificateAuthority as ::protobuf::Message>::default_instance()
    }
}

impl CertificateAuthority {
    pub fn new() -> CertificateAuthority {
        ::std::default::Default::default()
    }

    // .dev.sigstore.common.v1.DistinguishedName subject = 1;


    pub fn get_subject(&self) -> &super::sigstore_common::DistinguishedName {
        self.subject.as_ref().unwrap_or_else(|| <super::sigstore_common::DistinguishedName as ::protobuf::Message>::default_instance())
    }
    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    pub fn has_subject(&self) -> bool {
        self.subject.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: super::sigstore_common::DistinguishedName) {
        self.subject = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut super::sigstore_common::DistinguishedName {
        if self.subject.is_none() {
            self.subject.set_default();
        }
        self.subject.as_mut().unwrap()
    }

    // Take field
    pub fn take_subject(&mut self) -> super::sigstore_common::DistinguishedName {
        self.subject.take().unwrap_or_else(|| super::sigstore_common::DistinguishedName::new())
    }

    // string uri = 2;


    pub fn get_uri(&self) -> &str {
        &self.uri
    }
    pub fn clear_uri(&mut self) {
        self.uri.clear();
    }

    // Param is passed by value, moved
    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uri(&mut self) -> &mut ::std::string::String {
        &mut self.uri
    }

    // Take field
    pub fn take_uri(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.uri, ::std::string::String::new())
    }

    // .dev.sigstore.common.v1.X509CertificateChain cert_chain = 3;


    pub fn get_cert_chain(&self) -> &super::sigstore_common::X509CertificateChain {
        self.cert_chain.as_ref().unwrap_or_else(|| <super::sigstore_common::X509CertificateChain as ::protobuf::Message>::default_instance())
    }
    pub fn clear_cert_chain(&mut self) {
        self.cert_chain.clear();
    }

    pub fn has_cert_chain(&self) -> bool {
        self.cert_chain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cert_chain(&mut self, v: super::sigstore_common::X509CertificateChain) {
        self.cert_chain = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cert_chain(&mut self) -> &mut super::sigstore_common::X509CertificateChain {
        if self.cert_chain.is_none() {
            self.cert_chain.set_default();
        }
        self.cert_chain.as_mut().unwrap()
    }

    // Take field
    pub fn take_cert_chain(&mut self) -> super::sigstore_common::X509CertificateChain {
        self.cert_chain.take().unwrap_or_else(|| super::sigstore_common::X509CertificateChain::new())
    }

    // .dev.sigstore.common.v1.TimeRange valid_for = 4;


    pub fn get_valid_for(&self) -> &super::sigstore_common::TimeRange {
        self.valid_for.as_ref().unwrap_or_else(|| <super::sigstore_common::TimeRange as ::protobuf::Message>::default_instance())
    }
    pub fn clear_valid_for(&mut self) {
        self.valid_for.clear();
    }

    pub fn has_valid_for(&self) -> bool {
        self.valid_for.is_some()
    }

    // Param is passed by value, moved
    pub fn set_valid_for(&mut self, v: super::sigstore_common::TimeRange) {
        self.valid_for = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_valid_for(&mut self) -> &mut super::sigstore_common::TimeRange {
        if self.valid_for.is_none() {
            self.valid_for.set_default();
        }
        self.valid_for.as_mut().unwrap()
    }

    // Take field
    pub fn take_valid_for(&mut self) -> super::sigstore_common::TimeRange {
        self.valid_for.take().unwrap_or_else(|| super::sigstore_common::TimeRange::new())
    }
}

impl ::protobuf::Message for CertificateAuthority {
    fn is_initialized(&self) -> bool {
        for v in &self.subject {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cert_chain {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.valid_for {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.subject)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.uri)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cert_chain)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.valid_for)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.subject.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.uri.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.uri);
        }
        if let Some(ref v) = self.cert_chain.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.valid_for.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.subject.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.uri.is_empty() {
            os.write_string(2, &self.uri)?;
        }
        if let Some(ref v) = self.cert_chain.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.valid_for.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CertificateAuthority {
        CertificateAuthority::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::sigstore_common::DistinguishedName>>(
                "subject",
                |m: &CertificateAuthority| { &m.subject },
                |m: &mut CertificateAuthority| { &mut m.subject },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "uri",
                |m: &CertificateAuthority| { &m.uri },
                |m: &mut CertificateAuthority| { &mut m.uri },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::sigstore_common::X509CertificateChain>>(
                "cert_chain",
                |m: &CertificateAuthority| { &m.cert_chain },
                |m: &mut CertificateAuthority| { &mut m.cert_chain },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::sigstore_common::TimeRange>>(
                "valid_for",
                |m: &CertificateAuthority| { &m.valid_for },
                |m: &mut CertificateAuthority| { &mut m.valid_for },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CertificateAuthority>(
                "CertificateAuthority",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CertificateAuthority {
        static instance: ::protobuf::rt::LazyV2<CertificateAuthority> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CertificateAuthority::new)
    }
}

impl ::protobuf::Clear for CertificateAuthority {
    fn clear(&mut self) {
        self.subject.clear();
        self.uri.clear();
        self.cert_chain.clear();
        self.valid_for.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CertificateAuthority {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CertificateAuthority {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TrustedRoot {
    // message fields
    pub media_type: ::std::string::String,
    pub tlogs: ::protobuf::RepeatedField<TransparencyLogInstance>,
    pub certificate_authorities: ::protobuf::RepeatedField<CertificateAuthority>,
    pub ctlogs: ::protobuf::RepeatedField<TransparencyLogInstance>,
    pub timestamp_authorities: ::protobuf::RepeatedField<CertificateAuthority>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TrustedRoot {
    fn default() -> &'a TrustedRoot {
        <TrustedRoot as ::protobuf::Message>::default_instance()
    }
}

impl TrustedRoot {
    pub fn new() -> TrustedRoot {
        ::std::default::Default::default()
    }

    // string media_type = 1;


    pub fn get_media_type(&self) -> &str {
        &self.media_type
    }
    pub fn clear_media_type(&mut self) {
        self.media_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_media_type(&mut self, v: ::std::string::String) {
        self.media_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_media_type(&mut self) -> &mut ::std::string::String {
        &mut self.media_type
    }

    // Take field
    pub fn take_media_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.media_type, ::std::string::String::new())
    }

    // repeated .dev.sigstore.trustroot.v1.TransparencyLogInstance tlogs = 2;


    pub fn get_tlogs(&self) -> &[TransparencyLogInstance] {
        &self.tlogs
    }
    pub fn clear_tlogs(&mut self) {
        self.tlogs.clear();
    }

    // Param is passed by value, moved
    pub fn set_tlogs(&mut self, v: ::protobuf::RepeatedField<TransparencyLogInstance>) {
        self.tlogs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tlogs(&mut self) -> &mut ::protobuf::RepeatedField<TransparencyLogInstance> {
        &mut self.tlogs
    }

    // Take field
    pub fn take_tlogs(&mut self) -> ::protobuf::RepeatedField<TransparencyLogInstance> {
        ::std::mem::replace(&mut self.tlogs, ::protobuf::RepeatedField::new())
    }

    // repeated .dev.sigstore.trustroot.v1.CertificateAuthority certificate_authorities = 3;


    pub fn get_certificate_authorities(&self) -> &[CertificateAuthority] {
        &self.certificate_authorities
    }
    pub fn clear_certificate_authorities(&mut self) {
        self.certificate_authorities.clear();
    }

    // Param is passed by value, moved
    pub fn set_certificate_authorities(&mut self, v: ::protobuf::RepeatedField<CertificateAuthority>) {
        self.certificate_authorities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_certificate_authorities(&mut self) -> &mut ::protobuf::RepeatedField<CertificateAuthority> {
        &mut self.certificate_authorities
    }

    // Take field
    pub fn take_certificate_authorities(&mut self) -> ::protobuf::RepeatedField<CertificateAuthority> {
        ::std::mem::replace(&mut self.certificate_authorities, ::protobuf::RepeatedField::new())
    }

    // repeated .dev.sigstore.trustroot.v1.TransparencyLogInstance ctlogs = 4;


    pub fn get_ctlogs(&self) -> &[TransparencyLogInstance] {
        &self.ctlogs
    }
    pub fn clear_ctlogs(&mut self) {
        self.ctlogs.clear();
    }

    // Param is passed by value, moved
    pub fn set_ctlogs(&mut self, v: ::protobuf::RepeatedField<TransparencyLogInstance>) {
        self.ctlogs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ctlogs(&mut self) -> &mut ::protobuf::RepeatedField<TransparencyLogInstance> {
        &mut self.ctlogs
    }

    // Take field
    pub fn take_ctlogs(&mut self) -> ::protobuf::RepeatedField<TransparencyLogInstance> {
        ::std::mem::replace(&mut self.ctlogs, ::protobuf::RepeatedField::new())
    }

    // repeated .dev.sigstore.trustroot.v1.CertificateAuthority timestamp_authorities = 5;


    pub fn get_timestamp_authorities(&self) -> &[CertificateAuthority] {
        &self.timestamp_authorities
    }
    pub fn clear_timestamp_authorities(&mut self) {
        self.timestamp_authorities.clear();
    }

    // Param is passed by value, moved
    pub fn set_timestamp_authorities(&mut self, v: ::protobuf::RepeatedField<CertificateAuthority>) {
        self.timestamp_authorities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_timestamp_authorities(&mut self) -> &mut ::protobuf::RepeatedField<CertificateAuthority> {
        &mut self.timestamp_authorities
    }

    // Take field
    pub fn take_timestamp_authorities(&mut self) -> ::protobuf::RepeatedField<CertificateAuthority> {
        ::std::mem::replace(&mut self.timestamp_authorities, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for TrustedRoot {
    fn is_initialized(&self) -> bool {
        for v in &self.tlogs {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.certificate_authorities {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ctlogs {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.timestamp_authorities {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.media_type)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tlogs)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.certificate_authorities)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ctlogs)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.timestamp_authorities)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.media_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.media_type);
        }
        for value in &self.tlogs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.certificate_authorities {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.ctlogs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.timestamp_authorities {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.media_type.is_empty() {
            os.write_string(1, &self.media_type)?;
        }
        for v in &self.tlogs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.certificate_authorities {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.ctlogs {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.timestamp_authorities {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TrustedRoot {
        TrustedRoot::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "media_type",
                |m: &TrustedRoot| { &m.media_type },
                |m: &mut TrustedRoot| { &mut m.media_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TransparencyLogInstance>>(
                "tlogs",
                |m: &TrustedRoot| { &m.tlogs },
                |m: &mut TrustedRoot| { &mut m.tlogs },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CertificateAuthority>>(
                "certificate_authorities",
                |m: &TrustedRoot| { &m.certificate_authorities },
                |m: &mut TrustedRoot| { &mut m.certificate_authorities },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TransparencyLogInstance>>(
                "ctlogs",
                |m: &TrustedRoot| { &m.ctlogs },
                |m: &mut TrustedRoot| { &mut m.ctlogs },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CertificateAuthority>>(
                "timestamp_authorities",
                |m: &TrustedRoot| { &m.timestamp_authorities },
                |m: &mut TrustedRoot| { &mut m.timestamp_authorities },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TrustedRoot>(
                "TrustedRoot",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TrustedRoot {
        static instance: ::protobuf::rt::LazyV2<TrustedRoot> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TrustedRoot::new)
    }
}

impl ::protobuf::Clear for TrustedRoot {
    fn clear(&mut self) {
        self.media_type.clear();
        self.tlogs.clear();
        self.certificate_authorities.clear();
        self.ctlogs.clear();
        self.timestamp_authorities.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TrustedRoot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrustedRoot {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18sigstore_trustroot.proto\x12\x19dev.sigstore.trustroot.v1\x1a\x15s\
    igstore_common.proto\"\xfa\x01\n\x17TransparencyLogInstance\x12\x19\n\
    \x08base_url\x18\x01\x20\x01(\tR\x07baseUrl\x12L\n\x0ehash_algorithm\x18\
    \x02\x20\x01(\x0e2%.dev.sigstore.common.v1.HashAlgorithmR\rhashAlgorithm\
    \x12@\n\npublic_key\x18\x03\x20\x01(\x0b2!.dev.sigstore.common.v1.Public\
    KeyR\tpublicKey\x124\n\x06log_id\x18\x04\x20\x01(\x0b2\x1d.dev.sigstore.\
    common.v1.LogIdR\x05logId\"\xfa\x01\n\x14CertificateAuthority\x12C\n\x07\
    subject\x18\x01\x20\x01(\x0b2).dev.sigstore.common.v1.DistinguishedNameR\
    \x07subject\x12\x10\n\x03uri\x18\x02\x20\x01(\tR\x03uri\x12K\n\ncert_cha\
    in\x18\x03\x20\x01(\x0b2,.dev.sigstore.common.v1.X509CertificateChainR\t\
    certChain\x12>\n\tvalid_for\x18\x04\x20\x01(\x0b2!.dev.sigstore.common.v\
    1.TimeRangeR\x08validFor\"\x92\x03\n\x0bTrustedRoot\x12\x1d\n\nmedia_typ\
    e\x18\x01\x20\x01(\tR\tmediaType\x12H\n\x05tlogs\x18\x02\x20\x03(\x0b22.\
    dev.sigstore.trustroot.v1.TransparencyLogInstanceR\x05tlogs\x12h\n\x17ce\
    rtificate_authorities\x18\x03\x20\x03(\x0b2/.dev.sigstore.trustroot.v1.C\
    ertificateAuthorityR\x16certificateAuthorities\x12J\n\x06ctlogs\x18\x04\
    \x20\x03(\x0b22.dev.sigstore.trustroot.v1.TransparencyLogInstanceR\x06ct\
    logs\x12d\n\x15timestamp_authorities\x18\x05\x20\x03(\x0b2/.dev.sigstore\
    .trustroot.v1.CertificateAuthorityR\x14timestampAuthoritiesB\x88\x01\n\
    \x1fdev.sigstore.proto.trustroot.v1B\x0eTrustRootProtoP\x01Z9github.com/\
    sigstore/protobuf-specs/gen/pb-go/trustroot/v1\xea\x02\x17Sigstore::Trus\
    tRoot::V1b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
