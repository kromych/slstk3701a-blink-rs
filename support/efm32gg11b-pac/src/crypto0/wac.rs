#[doc = "Register `WAC` reader"]
pub type R = crate::R<WAC_SPEC>;
#[doc = "Register `WAC` writer"]
pub type W = crate::W<WAC_SPEC>;
#[doc = "Field `MODULUS` reader - Modular Operation Modulus"]
pub type MODULUS_R = crate::FieldReader<MODULUS_A>;
#[doc = "Modular Operation Modulus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODULUS_A {
    #[doc = "0: Generic modulus. p = 2^256"]
    BIN256 = 0,
    #[doc = "1: Generic modulus. p = 2^128"]
    BIN128 = 1,
    #[doc = "2: Modulus for B-233 and K-233 ECC curves. p(t) = t^233 + t^74 + 1"]
    ECCBIN233P = 2,
    #[doc = "3: Modulus for B-163 and K-163 ECC curves. p(t) = t^163 + t^7 + t^6 + t^3 + 1"]
    ECCBIN163P = 3,
    #[doc = "4: Modulus for GCM. P(t) = t^128 + t^7 + t^2 + t + 1"]
    GCMBIN128 = 4,
    #[doc = "5: Modulus for P-256 ECC curve. p = 2^256 - 2^224 + 2^192 + 2^96 - 1"]
    ECCPRIME256P = 5,
    #[doc = "6: Modulus for P-224 ECC curve. p = 2^224 - 2^96 - 1"]
    ECCPRIME224P = 6,
    #[doc = "7: Modulus for P-192 ECC curve. p = 2^192 - 2^64 - 1"]
    ECCPRIME192P = 7,
    #[doc = "8: P modulus for B-233 ECC curve"]
    ECCBIN233N = 8,
    #[doc = "9: P modulus for K-233 ECC curve"]
    ECCBIN233KN = 9,
    #[doc = "10: P modulus for B-163 ECC curve"]
    ECCBIN163N = 10,
    #[doc = "11: P modulus for K-163 ECC curve"]
    ECCBIN163KN = 11,
    #[doc = "12: P modulus for P-256 ECC curve"]
    ECCPRIME256N = 12,
    #[doc = "13: P modulus for P-224 ECC curve"]
    ECCPRIME224N = 13,
    #[doc = "14: P modulus for P-192 ECC curve"]
    ECCPRIME192N = 14,
}
impl From<MODULUS_A> for u8 {
    #[inline(always)]
    fn from(variant: MODULUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODULUS_A {
    type Ux = u8;
}
impl MODULUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODULUS_A> {
        match self.bits {
            0 => Some(MODULUS_A::BIN256),
            1 => Some(MODULUS_A::BIN128),
            2 => Some(MODULUS_A::ECCBIN233P),
            3 => Some(MODULUS_A::ECCBIN163P),
            4 => Some(MODULUS_A::GCMBIN128),
            5 => Some(MODULUS_A::ECCPRIME256P),
            6 => Some(MODULUS_A::ECCPRIME224P),
            7 => Some(MODULUS_A::ECCPRIME192P),
            8 => Some(MODULUS_A::ECCBIN233N),
            9 => Some(MODULUS_A::ECCBIN233KN),
            10 => Some(MODULUS_A::ECCBIN163N),
            11 => Some(MODULUS_A::ECCBIN163KN),
            12 => Some(MODULUS_A::ECCPRIME256N),
            13 => Some(MODULUS_A::ECCPRIME224N),
            14 => Some(MODULUS_A::ECCPRIME192N),
            _ => None,
        }
    }
    #[doc = "Generic modulus. p = 2^256"]
    #[inline(always)]
    pub fn is_bin256(&self) -> bool {
        *self == MODULUS_A::BIN256
    }
    #[doc = "Generic modulus. p = 2^128"]
    #[inline(always)]
    pub fn is_bin128(&self) -> bool {
        *self == MODULUS_A::BIN128
    }
    #[doc = "Modulus for B-233 and K-233 ECC curves. p(t) = t^233 + t^74 + 1"]
    #[inline(always)]
    pub fn is_eccbin233p(&self) -> bool {
        *self == MODULUS_A::ECCBIN233P
    }
    #[doc = "Modulus for B-163 and K-163 ECC curves. p(t) = t^163 + t^7 + t^6 + t^3 + 1"]
    #[inline(always)]
    pub fn is_eccbin163p(&self) -> bool {
        *self == MODULUS_A::ECCBIN163P
    }
    #[doc = "Modulus for GCM. P(t) = t^128 + t^7 + t^2 + t + 1"]
    #[inline(always)]
    pub fn is_gcmbin128(&self) -> bool {
        *self == MODULUS_A::GCMBIN128
    }
    #[doc = "Modulus for P-256 ECC curve. p = 2^256 - 2^224 + 2^192 + 2^96 - 1"]
    #[inline(always)]
    pub fn is_eccprime256p(&self) -> bool {
        *self == MODULUS_A::ECCPRIME256P
    }
    #[doc = "Modulus for P-224 ECC curve. p = 2^224 - 2^96 - 1"]
    #[inline(always)]
    pub fn is_eccprime224p(&self) -> bool {
        *self == MODULUS_A::ECCPRIME224P
    }
    #[doc = "Modulus for P-192 ECC curve. p = 2^192 - 2^64 - 1"]
    #[inline(always)]
    pub fn is_eccprime192p(&self) -> bool {
        *self == MODULUS_A::ECCPRIME192P
    }
    #[doc = "P modulus for B-233 ECC curve"]
    #[inline(always)]
    pub fn is_eccbin233n(&self) -> bool {
        *self == MODULUS_A::ECCBIN233N
    }
    #[doc = "P modulus for K-233 ECC curve"]
    #[inline(always)]
    pub fn is_eccbin233kn(&self) -> bool {
        *self == MODULUS_A::ECCBIN233KN
    }
    #[doc = "P modulus for B-163 ECC curve"]
    #[inline(always)]
    pub fn is_eccbin163n(&self) -> bool {
        *self == MODULUS_A::ECCBIN163N
    }
    #[doc = "P modulus for K-163 ECC curve"]
    #[inline(always)]
    pub fn is_eccbin163kn(&self) -> bool {
        *self == MODULUS_A::ECCBIN163KN
    }
    #[doc = "P modulus for P-256 ECC curve"]
    #[inline(always)]
    pub fn is_eccprime256n(&self) -> bool {
        *self == MODULUS_A::ECCPRIME256N
    }
    #[doc = "P modulus for P-224 ECC curve"]
    #[inline(always)]
    pub fn is_eccprime224n(&self) -> bool {
        *self == MODULUS_A::ECCPRIME224N
    }
    #[doc = "P modulus for P-192 ECC curve"]
    #[inline(always)]
    pub fn is_eccprime192n(&self) -> bool {
        *self == MODULUS_A::ECCPRIME192N
    }
}
#[doc = "Field `MODULUS` writer - Modular Operation Modulus"]
pub type MODULUS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODULUS_A>;
impl<'a, REG> MODULUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generic modulus. p = 2^256"]
    #[inline(always)]
    pub fn bin256(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::BIN256)
    }
    #[doc = "Generic modulus. p = 2^128"]
    #[inline(always)]
    pub fn bin128(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::BIN128)
    }
    #[doc = "Modulus for B-233 and K-233 ECC curves. p(t) = t^233 + t^74 + 1"]
    #[inline(always)]
    pub fn eccbin233p(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCBIN233P)
    }
    #[doc = "Modulus for B-163 and K-163 ECC curves. p(t) = t^163 + t^7 + t^6 + t^3 + 1"]
    #[inline(always)]
    pub fn eccbin163p(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCBIN163P)
    }
    #[doc = "Modulus for GCM. P(t) = t^128 + t^7 + t^2 + t + 1"]
    #[inline(always)]
    pub fn gcmbin128(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::GCMBIN128)
    }
    #[doc = "Modulus for P-256 ECC curve. p = 2^256 - 2^224 + 2^192 + 2^96 - 1"]
    #[inline(always)]
    pub fn eccprime256p(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCPRIME256P)
    }
    #[doc = "Modulus for P-224 ECC curve. p = 2^224 - 2^96 - 1"]
    #[inline(always)]
    pub fn eccprime224p(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCPRIME224P)
    }
    #[doc = "Modulus for P-192 ECC curve. p = 2^192 - 2^64 - 1"]
    #[inline(always)]
    pub fn eccprime192p(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCPRIME192P)
    }
    #[doc = "P modulus for B-233 ECC curve"]
    #[inline(always)]
    pub fn eccbin233n(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCBIN233N)
    }
    #[doc = "P modulus for K-233 ECC curve"]
    #[inline(always)]
    pub fn eccbin233kn(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCBIN233KN)
    }
    #[doc = "P modulus for B-163 ECC curve"]
    #[inline(always)]
    pub fn eccbin163n(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCBIN163N)
    }
    #[doc = "P modulus for K-163 ECC curve"]
    #[inline(always)]
    pub fn eccbin163kn(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCBIN163KN)
    }
    #[doc = "P modulus for P-256 ECC curve"]
    #[inline(always)]
    pub fn eccprime256n(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCPRIME256N)
    }
    #[doc = "P modulus for P-224 ECC curve"]
    #[inline(always)]
    pub fn eccprime224n(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCPRIME224N)
    }
    #[doc = "P modulus for P-192 ECC curve"]
    #[inline(always)]
    pub fn eccprime192n(self) -> &'a mut crate::W<REG> {
        self.variant(MODULUS_A::ECCPRIME192N)
    }
}
#[doc = "Field `MODOP` reader - Modular Operation Field Type"]
pub type MODOP_R = crate::BitReader;
#[doc = "Field `MODOP` writer - Modular Operation Field Type"]
pub type MODOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULWIDTH` reader - Multiply Width"]
pub type MULWIDTH_R = crate::FieldReader<MULWIDTH_A>;
#[doc = "Multiply Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULWIDTH_A {
    #[doc = "0: Multiply 256 bits"]
    MUL256 = 0,
    #[doc = "1: Multiply 128 bits"]
    MUL128 = 1,
    #[doc = "2: Same number of bits as specified by MODULUS"]
    MULMOD = 2,
}
impl From<MULWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: MULWIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MULWIDTH_A {
    type Ux = u8;
}
impl MULWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MULWIDTH_A> {
        match self.bits {
            0 => Some(MULWIDTH_A::MUL256),
            1 => Some(MULWIDTH_A::MUL128),
            2 => Some(MULWIDTH_A::MULMOD),
            _ => None,
        }
    }
    #[doc = "Multiply 256 bits"]
    #[inline(always)]
    pub fn is_mul256(&self) -> bool {
        *self == MULWIDTH_A::MUL256
    }
    #[doc = "Multiply 128 bits"]
    #[inline(always)]
    pub fn is_mul128(&self) -> bool {
        *self == MULWIDTH_A::MUL128
    }
    #[doc = "Same number of bits as specified by MODULUS"]
    #[inline(always)]
    pub fn is_mulmod(&self) -> bool {
        *self == MULWIDTH_A::MULMOD
    }
}
#[doc = "Field `MULWIDTH` writer - Multiply Width"]
pub type MULWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MULWIDTH_A>;
impl<'a, REG> MULWIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Multiply 256 bits"]
    #[inline(always)]
    pub fn mul256(self) -> &'a mut crate::W<REG> {
        self.variant(MULWIDTH_A::MUL256)
    }
    #[doc = "Multiply 128 bits"]
    #[inline(always)]
    pub fn mul128(self) -> &'a mut crate::W<REG> {
        self.variant(MULWIDTH_A::MUL128)
    }
    #[doc = "Same number of bits as specified by MODULUS"]
    #[inline(always)]
    pub fn mulmod(self) -> &'a mut crate::W<REG> {
        self.variant(MULWIDTH_A::MULMOD)
    }
}
#[doc = "Field `RESULTWIDTH` reader - Result Width"]
pub type RESULTWIDTH_R = crate::FieldReader<RESULTWIDTH_A>;
#[doc = "Result Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESULTWIDTH_A {
    #[doc = "0: Results have 256 bits"]
    _256BIT = 0,
    #[doc = "1: Results have 128 bits"]
    _128BIT = 1,
    #[doc = "2: Results have 260 bits. Upper bits of result can be read through DDATA0MSBS in CRYPTO_STATUS"]
    _260BIT = 2,
}
impl From<RESULTWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: RESULTWIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RESULTWIDTH_A {
    type Ux = u8;
}
impl RESULTWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RESULTWIDTH_A> {
        match self.bits {
            0 => Some(RESULTWIDTH_A::_256BIT),
            1 => Some(RESULTWIDTH_A::_128BIT),
            2 => Some(RESULTWIDTH_A::_260BIT),
            _ => None,
        }
    }
    #[doc = "Results have 256 bits"]
    #[inline(always)]
    pub fn is_256bit(&self) -> bool {
        *self == RESULTWIDTH_A::_256BIT
    }
    #[doc = "Results have 128 bits"]
    #[inline(always)]
    pub fn is_128bit(&self) -> bool {
        *self == RESULTWIDTH_A::_128BIT
    }
    #[doc = "Results have 260 bits. Upper bits of result can be read through DDATA0MSBS in CRYPTO_STATUS"]
    #[inline(always)]
    pub fn is_260bit(&self) -> bool {
        *self == RESULTWIDTH_A::_260BIT
    }
}
#[doc = "Field `RESULTWIDTH` writer - Result Width"]
pub type RESULTWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RESULTWIDTH_A>;
impl<'a, REG> RESULTWIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Results have 256 bits"]
    #[inline(always)]
    pub fn _256bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESULTWIDTH_A::_256BIT)
    }
    #[doc = "Results have 128 bits"]
    #[inline(always)]
    pub fn _128bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESULTWIDTH_A::_128BIT)
    }
    #[doc = "Results have 260 bits. Upper bits of result can be read through DDATA0MSBS in CRYPTO_STATUS"]
    #[inline(always)]
    pub fn _260bit(self) -> &'a mut crate::W<REG> {
        self.variant(RESULTWIDTH_A::_260BIT)
    }
}
impl R {
    #[doc = "Bits 0:3 - Modular Operation Modulus"]
    #[inline(always)]
    pub fn modulus(&self) -> MODULUS_R {
        MODULUS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Modular Operation Field Type"]
    #[inline(always)]
    pub fn modop(&self) -> MODOP_R {
        MODOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Multiply Width"]
    #[inline(always)]
    pub fn mulwidth(&self) -> MULWIDTH_R {
        MULWIDTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Result Width"]
    #[inline(always)]
    pub fn resultwidth(&self) -> RESULTWIDTH_R {
        RESULTWIDTH_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Modular Operation Modulus"]
    #[inline(always)]
    #[must_use]
    pub fn modulus(&mut self) -> MODULUS_W<WAC_SPEC> {
        MODULUS_W::new(self, 0)
    }
    #[doc = "Bit 4 - Modular Operation Field Type"]
    #[inline(always)]
    #[must_use]
    pub fn modop(&mut self) -> MODOP_W<WAC_SPEC> {
        MODOP_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Multiply Width"]
    #[inline(always)]
    #[must_use]
    pub fn mulwidth(&mut self) -> MULWIDTH_W<WAC_SPEC> {
        MULWIDTH_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Result Width"]
    #[inline(always)]
    #[must_use]
    pub fn resultwidth(&mut self) -> RESULTWIDTH_W<WAC_SPEC> {
        RESULTWIDTH_W::new(self, 10)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Wide Arithmetic Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAC_SPEC;
impl crate::RegisterSpec for WAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wac::R`](R) reader structure"]
impl crate::Readable for WAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wac::W`](W) writer structure"]
impl crate::Writable for WAC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAC to value 0"]
impl crate::Resettable for WAC_SPEC {
    const RESET_VALUE: u32 = 0;
}
