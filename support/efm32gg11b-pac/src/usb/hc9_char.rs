#[doc = "Register `HC9_CHAR` reader"]
pub type R = crate::R<Hc9CharSpec>;
#[doc = "Register `HC9_CHAR` writer"]
pub type W = crate::W<Hc9CharSpec>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MpsR = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNUM` reader - Endpoint Number"]
pub type EpnumR = crate::FieldReader;
#[doc = "Field `EPNUM` writer - Endpoint Number"]
pub type EpnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - Endpoint Direction"]
pub type EpdirR = crate::BitReader;
#[doc = "Field `EPDIR` writer - Endpoint Direction"]
pub type EpdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPDDEV` reader - Low-Speed Device"]
pub type LspddevR = crate::BitReader;
#[doc = "Field `LSPDDEV` writer - Low-Speed Device"]
pub type LspddevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eptype {
    #[doc = "0: Control endpoint."]
    Control = 0,
    #[doc = "1: Isochronous endpoint."]
    Iso = 1,
    #[doc = "2: Bulk endpoint."]
    Bulk = 2,
    #[doc = "3: Interrupt endpoint."]
    Int = 3,
}
impl From<Eptype> for u8 {
    #[inline(always)]
    fn from(variant: Eptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eptype {
    type Ux = u8;
}
impl crate::IsEnum for Eptype {}
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader<Eptype>;
impl EptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eptype {
        match self.bits {
            0 => Eptype::Control,
            1 => Eptype::Iso,
            2 => Eptype::Bulk,
            3 => Eptype::Int,
            _ => unreachable!(),
        }
    }
    #[doc = "Control endpoint."]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == Eptype::Control
    }
    #[doc = "Isochronous endpoint."]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Eptype::Iso
    }
    #[doc = "Bulk endpoint."]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == Eptype::Bulk
    }
    #[doc = "Interrupt endpoint."]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Eptype::Int
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eptype, crate::Safe>;
impl<'a, REG> EptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control endpoint."]
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Control)
    }
    #[doc = "Isochronous endpoint."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Iso)
    }
    #[doc = "Bulk endpoint."]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Bulk)
    }
    #[doc = "Interrupt endpoint."]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Int)
    }
}
#[doc = "Field `MC` reader - Multi Count (MC) / Error Count"]
pub type McR = crate::FieldReader;
#[doc = "Field `MC` writer - Multi Count (MC) / Error Count"]
pub type McW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVADDR` reader - Device Address"]
pub type DevaddrR = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device Address"]
pub type DevaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - Odd Frame"]
pub type OddfrmR = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd Frame"]
pub type OddfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDIS` reader - Channel Disable"]
pub type ChdisR = crate::BitReader;
#[doc = "Field `CHDIS` writer - Channel Disable"]
pub type ChdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENA` reader - Channel Enable"]
pub type ChenaR = crate::BitReader;
#[doc = "Field `CHENA` writer - Channel Enable"]
pub type ChenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EpdirR {
        EpdirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-Speed Device"]
    #[inline(always)]
    pub fn lspddev(&self) -> LspddevR {
        LspddevR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DevaddrR {
        DevaddrR::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> OddfrmR {
        OddfrmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn chdis(&self) -> ChdisR {
        ChdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn chena(&self) -> ChenaR {
        ChenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MpsW<'_, Hc9CharSpec> {
        MpsW::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EpnumW<'_, Hc9CharSpec> {
        EpnumW::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EpdirW<'_, Hc9CharSpec> {
        EpdirW::new(self, 15)
    }
    #[doc = "Bit 17 - Low-Speed Device"]
    #[inline(always)]
    pub fn lspddev(&mut self) -> LspddevW<'_, Hc9CharSpec> {
        LspddevW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EptypeW<'_, Hc9CharSpec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count"]
    #[inline(always)]
    pub fn mc(&mut self) -> McW<'_, Hc9CharSpec> {
        McW::new(self, 20)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DevaddrW<'_, Hc9CharSpec> {
        DevaddrW::new(self, 22)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn oddfrm(&mut self) -> OddfrmW<'_, Hc9CharSpec> {
        OddfrmW::new(self, 29)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn chdis(&mut self) -> ChdisW<'_, Hc9CharSpec> {
        ChdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn chena(&mut self) -> ChenaW<'_, Hc9CharSpec> {
        ChenaW::new(self, 31)
    }
}
#[doc = "Host Channel x Characteristics Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc9_char::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc9_char::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc9CharSpec;
impl crate::RegisterSpec for Hc9CharSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc9_char::R`](R) reader structure"]
impl crate::Readable for Hc9CharSpec {}
#[doc = "`write(|w| ..)` method takes [`hc9_char::W`](W) writer structure"]
impl crate::Writable for Hc9CharSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HC9_CHAR to value 0"]
impl crate::Resettable for Hc9CharSpec {}
