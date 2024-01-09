#[doc = "Register `HC3_TSIZ` reader"]
pub type R = crate::R<HC3_TSIZ_SPEC>;
#[doc = "Register `HC3_TSIZ` writer"]
pub type W = crate::W<HC3_TSIZ_SPEC>;
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PID` reader - The Application Programs This Field With the Type of"]
pub type PID_R = crate::FieldReader<PID_A>;
#[doc = "The Application Programs This Field With the Type of\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PID_A {
    #[doc = "0: DATA0 PID."]
    DATA0 = 0,
    #[doc = "1: DATA2 PID."]
    DATA2 = 1,
    #[doc = "2: DATA1 PID."]
    DATA1 = 2,
    #[doc = "3: MDATA (non-control) / SETUP (control) PID."]
    MDATA = 3,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PID_A {
    type Ux = u8;
}
impl PID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PID_A {
        match self.bits {
            0 => PID_A::DATA0,
            1 => PID_A::DATA2,
            2 => PID_A::DATA1,
            3 => PID_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0 PID."]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == PID_A::DATA0
    }
    #[doc = "DATA2 PID."]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == PID_A::DATA2
    }
    #[doc = "DATA1 PID."]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == PID_A::DATA1
    }
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == PID_A::MDATA
    }
}
#[doc = "Field `PID` writer - The Application Programs This Field With the Type of"]
pub type PID_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PID_A>;
impl<'a, REG> PID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DATA0 PID."]
    #[inline(always)]
    pub fn data0(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::DATA0)
    }
    #[doc = "DATA2 PID."]
    #[inline(always)]
    pub fn data2(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::DATA2)
    }
    #[doc = "DATA1 PID."]
    #[inline(always)]
    pub fn data1(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::DATA1)
    }
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    #[inline(always)]
    pub fn mdata(self) -> &'a mut crate::W<REG> {
        self.variant(PID_A::MDATA)
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - The Application Programs This Field With the Type of"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<HC3_TSIZ_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<HC3_TSIZ_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - The Application Programs This Field With the Type of"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<HC3_TSIZ_SPEC> {
        PID_W::new(self, 29)
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
#[doc = "Host Channel x Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc3_tsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc3_tsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC3_TSIZ_SPEC;
impl crate::RegisterSpec for HC3_TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc3_tsiz::R`](R) reader structure"]
impl crate::Readable for HC3_TSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc3_tsiz::W`](W) writer structure"]
impl crate::Writable for HC3_TSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HC3_TSIZ to value 0"]
impl crate::Resettable for HC3_TSIZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
