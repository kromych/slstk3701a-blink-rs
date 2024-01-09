#[doc = "Register `HC1_CHAR` reader"]
pub type R = crate::R<HC1_CHAR_SPEC>;
#[doc = "Register `HC1_CHAR` writer"]
pub type W = crate::W<HC1_CHAR_SPEC>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNUM` reader - Endpoint Number"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `EPNUM` writer - Endpoint Number"]
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - Endpoint Direction"]
pub type EPDIR_R = crate::BitReader;
#[doc = "Field `EPDIR` writer - Endpoint Direction"]
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPDDEV` reader - Low-Speed Device"]
pub type LSPDDEV_R = crate::BitReader;
#[doc = "Field `LSPDDEV` writer - Low-Speed Device"]
pub type LSPDDEV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<EPTYPE_A>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control endpoint."]
    CONTROL = 0,
    #[doc = "1: Isochronous endpoint."]
    ISO = 1,
    #[doc = "2: Bulk endpoint."]
    BULK = 2,
    #[doc = "3: Interrupt endpoint."]
    INT = 3,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPTYPE_A {
    type Ux = u8;
}
impl EPTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPTYPE_A {
        match self.bits {
            0 => EPTYPE_A::CONTROL,
            1 => EPTYPE_A::ISO,
            2 => EPTYPE_A::BULK,
            3 => EPTYPE_A::INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Control endpoint."]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == EPTYPE_A::CONTROL
    }
    #[doc = "Isochronous endpoint."]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EPTYPE_A::ISO
    }
    #[doc = "Bulk endpoint."]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EPTYPE_A::BULK
    }
    #[doc = "Interrupt endpoint."]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == EPTYPE_A::INT
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EPTYPE_A>;
impl<'a, REG> EPTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control endpoint."]
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::CONTROL)
    }
    #[doc = "Isochronous endpoint."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::ISO)
    }
    #[doc = "Bulk endpoint."]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::BULK)
    }
    #[doc = "Interrupt endpoint."]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::INT)
    }
}
#[doc = "Field `MC` reader - Multi Count (MC) / Error Count"]
pub type MC_R = crate::FieldReader;
#[doc = "Field `MC` writer - Multi Count (MC) / Error Count"]
pub type MC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVADDR` reader - Device Address"]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device Address"]
pub type DEVADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - Odd Frame"]
pub type ODDFRM_R = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd Frame"]
pub type ODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDIS` reader - Channel Disable"]
pub type CHDIS_R = crate::BitReader;
#[doc = "Field `CHDIS` writer - Channel Disable"]
pub type CHDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENA` reader - Channel Enable"]
pub type CHENA_R = crate::BitReader;
#[doc = "Field `CHENA` writer - Channel Enable"]
pub type CHENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-Speed Device"]
    #[inline(always)]
    pub fn lspddev(&self) -> LSPDDEV_R {
        LSPDDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<HC1_CHAR_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<HC1_CHAR_SPEC> {
        EPNUM_W::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<HC1_CHAR_SPEC> {
        EPDIR_W::new(self, 15)
    }
    #[doc = "Bit 17 - Low-Speed Device"]
    #[inline(always)]
    #[must_use]
    pub fn lspddev(&mut self) -> LSPDDEV_W<HC1_CHAR_SPEC> {
        LSPDDEV_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<HC1_CHAR_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count"]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<HC1_CHAR_SPEC> {
        MC_W::new(self, 20)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DEVADDR_W<HC1_CHAR_SPEC> {
        DEVADDR_W::new(self, 22)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> ODDFRM_W<HC1_CHAR_SPEC> {
        ODDFRM_W::new(self, 29)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chdis(&mut self) -> CHDIS_W<HC1_CHAR_SPEC> {
        CHDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chena(&mut self) -> CHENA_W<HC1_CHAR_SPEC> {
        CHENA_W::new(self, 31)
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
#[doc = "Host Channel x Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1_char::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1_char::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC1_CHAR_SPEC;
impl crate::RegisterSpec for HC1_CHAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc1_char::R`](R) reader structure"]
impl crate::Readable for HC1_CHAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc1_char::W`](W) writer structure"]
impl crate::Writable for HC1_CHAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HC1_CHAR to value 0"]
impl crate::Resettable for HC1_CHAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
