#[doc = "Register `PAGECTRL` reader"]
pub type R = crate::R<PAGECTRL_SPEC>;
#[doc = "Register `PAGECTRL` writer"]
pub type W = crate::W<PAGECTRL_SPEC>;
#[doc = "Field `PAGELEN` reader - Page Length"]
pub type PAGELEN_R = crate::FieldReader<PAGELEN_A>;
#[doc = "Page Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PAGELEN_A {
    #[doc = "0: 4 members in a page."]
    MEMBER4 = 0,
    #[doc = "1: 8 members in a page."]
    MEMBER8 = 1,
    #[doc = "2: 16 members in a page."]
    MEMBER16 = 2,
    #[doc = "3: 32 members in a page."]
    MEMBER32 = 3,
}
impl From<PAGELEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PAGELEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PAGELEN_A {
    type Ux = u8;
}
impl PAGELEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGELEN_A {
        match self.bits {
            0 => PAGELEN_A::MEMBER4,
            1 => PAGELEN_A::MEMBER8,
            2 => PAGELEN_A::MEMBER16,
            3 => PAGELEN_A::MEMBER32,
            _ => unreachable!(),
        }
    }
    #[doc = "4 members in a page."]
    #[inline(always)]
    pub fn is_member4(&self) -> bool {
        *self == PAGELEN_A::MEMBER4
    }
    #[doc = "8 members in a page."]
    #[inline(always)]
    pub fn is_member8(&self) -> bool {
        *self == PAGELEN_A::MEMBER8
    }
    #[doc = "16 members in a page."]
    #[inline(always)]
    pub fn is_member16(&self) -> bool {
        *self == PAGELEN_A::MEMBER16
    }
    #[doc = "32 members in a page."]
    #[inline(always)]
    pub fn is_member32(&self) -> bool {
        *self == PAGELEN_A::MEMBER32
    }
}
#[doc = "Field `PAGELEN` writer - Page Length"]
pub type PAGELEN_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PAGELEN_A>;
impl<'a, REG, const O: u8> PAGELEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 members in a page."]
    #[inline(always)]
    pub fn member4(self) -> &'a mut crate::W<REG> {
        self.variant(PAGELEN_A::MEMBER4)
    }
    #[doc = "8 members in a page."]
    #[inline(always)]
    pub fn member8(self) -> &'a mut crate::W<REG> {
        self.variant(PAGELEN_A::MEMBER8)
    }
    #[doc = "16 members in a page."]
    #[inline(always)]
    pub fn member16(self) -> &'a mut crate::W<REG> {
        self.variant(PAGELEN_A::MEMBER16)
    }
    #[doc = "32 members in a page."]
    #[inline(always)]
    pub fn member32(self) -> &'a mut crate::W<REG> {
        self.variant(PAGELEN_A::MEMBER32)
    }
}
#[doc = "Field `INCHIT` reader - Intrapage Hit Only on Incremental Addresses"]
pub type INCHIT_R = crate::BitReader;
#[doc = "Field `INCHIT` writer - Intrapage Hit Only on Incremental Addresses"]
pub type INCHIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDPA` reader - Page Read Access Time"]
pub type RDPA_R = crate::FieldReader;
#[doc = "Field `RDPA` writer - Page Read Access Time"]
pub type RDPA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `KEEPOPEN` reader - Maximum Page Open Time"]
pub type KEEPOPEN_R = crate::FieldReader;
#[doc = "Field `KEEPOPEN` writer - Maximum Page Open Time"]
pub type KEEPOPEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:1 - Page Length"]
    #[inline(always)]
    pub fn pagelen(&self) -> PAGELEN_R {
        PAGELEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Intrapage Hit Only on Incremental Addresses"]
    #[inline(always)]
    pub fn inchit(&self) -> INCHIT_R {
        INCHIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Page Read Access Time"]
    #[inline(always)]
    pub fn rdpa(&self) -> RDPA_R {
        RDPA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 20:26 - Maximum Page Open Time"]
    #[inline(always)]
    pub fn keepopen(&self) -> KEEPOPEN_R {
        KEEPOPEN_R::new(((self.bits >> 20) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Page Length"]
    #[inline(always)]
    #[must_use]
    pub fn pagelen(&mut self) -> PAGELEN_W<PAGECTRL_SPEC, 0> {
        PAGELEN_W::new(self)
    }
    #[doc = "Bit 4 - Intrapage Hit Only on Incremental Addresses"]
    #[inline(always)]
    #[must_use]
    pub fn inchit(&mut self) -> INCHIT_W<PAGECTRL_SPEC, 4> {
        INCHIT_W::new(self)
    }
    #[doc = "Bits 8:11 - Page Read Access Time"]
    #[inline(always)]
    #[must_use]
    pub fn rdpa(&mut self) -> RDPA_W<PAGECTRL_SPEC, 8> {
        RDPA_W::new(self)
    }
    #[doc = "Bits 20:26 - Maximum Page Open Time"]
    #[inline(always)]
    #[must_use]
    pub fn keepopen(&mut self) -> KEEPOPEN_W<PAGECTRL_SPEC, 20> {
        KEEPOPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Page Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pagectrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pagectrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAGECTRL_SPEC;
impl crate::RegisterSpec for PAGECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pagectrl::R`](R) reader structure"]
impl crate::Readable for PAGECTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pagectrl::W`](W) writer structure"]
impl crate::Writable for PAGECTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAGECTRL to value 0x0f00"]
impl crate::Resettable for PAGECTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00;
}
